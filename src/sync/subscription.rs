use crate::sync::client::REDIS_PAYLOAD_KEY;
use crate::sync::types::{Payload, Subscription};
use crossbeam_channel::{bounded, select, Receiver, Sender};
use log::{debug, error, info, warn};
use redis::{Client as RedisClient, Commands, Connection, FromRedisValue, Value};
use std::collections::{HashMap, HashSet};
use std::thread::spawn;
use std::time::Duration;

// TODO Store conn inside hashmap is a bad idea

pub(crate) fn start_subscription_handler(mut redis_client: RedisClient) -> Sender<Subscription> {
    let (add_sub_sender, add_sub_receiver) = bounded(100);
    spawn(move || {
        run_subscription_handler(redis_client, add_sub_receiver);
    });
    add_sub_sender
}

fn run_subscription_handler(
    mut redis_client: RedisClient,
    add_sub_receiver: Receiver<Subscription>,
) {
    let mut actives = HashMap::new();

    // let consumer = SubscriptionConsumer::new();
    loop {
        manage_subscriptions(&mut actives, &add_sub_receiver);
        if !actives.is_empty() {
            consume_subscriptions(&mut redis_client, &mut actives);
        }
    }
}

fn manage_subscriptions(
    actives: &mut HashMap<String, Subscription>,
    add_sub_receiver: &Receiver<Subscription>,
) {
    loop {
        select! {
            recv(add_sub_receiver) -> msg => {
              if let Ok(subscription) = msg {
                  let key = subscription.redis_key();
                  if actives.contains_key(&key) {
                    if subscription.response().send(Err("failed to add duplicate subscription".to_string())).is_err() {
                      debug!("duplicated subscription {:?}", subscription);
                    }
                    continue
                  }

                  info!("SUBSCRIBE {}", key);
                  actives.insert(key, subscription);
              }
            }
            default => {
              return;
            }
        }
    }
}

fn consume_subscriptions(
    redis_client: &mut RedisClient,
    actives: &mut HashMap<String, Subscription>,
) {
    let mut cmd = redis::cmd("XREAD");
    cmd.arg("COUNT").arg(10); // max 10 elements per stream
    cmd.arg("BLOCK").arg(0); // "BLOCK 0 `of 0 means to never timeout, block forever if no elements are available
    cmd.arg("STREAMS");
    for sub in actives.values() {
        cmd.arg(sub.redis_key()).arg(sub.last_id());
    }

    // Vec<Vec<channel, Vec<msg_id, HashMap<redis_payload_key, payload>>>>>
    let res: Result<Vec<Vec<Value>>, _> = cmd.query(redis_client);
    // let res: Result<Vec<(String, Vec<(String, Vec<(String, String)>)>)>,_> = cmd.query(redis_client);
    // match cmd.query::<Vec<(String, Vec<(String, Vec<(String, String)>)>)>>(redis_client) {
    match res {
        Err(err) => {
            error!("failed to XREAD: {:?}", err);
            return;
        }
        Ok(streams) => {
            let mut removals = HashSet::new();

            for stream in streams {
                let channel = String::from_redis_value(&stream[0]).expect("stream channel");
                let msgs =
                    Vec::<Vec<Value>>::from_redis_value(&stream[1]).expect("stream messages"); // stream[1];
                let sub = actives.get_mut(&channel).expect("subscribing channel");
                let mut peer_closed = false;

                for msg in msgs {
                    let msg_id = String::from_redis_value(&msg[0]).expect("message id");
                    let entries = Vec::<(String, Payload)>::from_redis_value(&msg[1])
                        .expect("message content");
                    for (entry_id, payload) in entries {
                        if &entry_id == REDIS_PAYLOAD_KEY {
                            if sub.response().send(Ok(payload)).is_err() {
                                // we could not send value because context fired.
                                // skip all further messages on this stream, and queue for
                                // removal.
                                removals.insert(channel.clone());
                                break;
                            }
                            sub.set_last_id(msg_id.clone());
                            break;
                        }
                    }
                }
            }

            if !removals.is_empty() {
                for channel in removals {
                    actives.remove(&channel);
                }
            }
        }
    }
}
