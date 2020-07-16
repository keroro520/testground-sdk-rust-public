use crossbeam_channel::{bounded, select, Receiver, Sender};
use log::{error, debug,warn};
use redis::{Client as RedisClient, Commands, Connection};
use std::collections::HashMap;
use crate::sync::types::Subscription;
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

fn run_subscription_handler(mut redis_client: RedisClient, add_sub_receiver: Receiver<Subscription>) {
    let mut actives = HashMap::new();

    // let consumer = SubscriptionConsumer::new();
    loop {
        manage_subscriptions(&mut redis_client, &mut actives, &add_sub_receiver);
        if !actives.is_empty() {
            println!("bilibili 111");
            consume_subscriptions(&mut redis_client, &mut actives);
            println!("bilibili 222");
        }
    }
}

fn manage_subscriptions(
    redis_client: &mut RedisClient,
    actives: &mut HashMap<String, (Connection, Subscription)>,
    add_sub_receiver: &Receiver<Subscription>) {
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

                let mut conn = match redis_client.get_connection() {
                    Ok(conn) => {
                        conn
                    }
                    Err(err) => {
                        warn!("failed to get connection; iteration skipped, error {:?}", err);
                        return;
                    }
                };

                println!("SUBSCRIBE {}", key);
                conn.set_read_timeout(Some(Duration::from_secs(2)));
                let mut pubsub = conn.as_pubsub();
                pubsub.subscribe(&key).expect(&format!("failed to subscribe topic \"{}\"", key));
                loop {

                    println!(
                        "bilibili {:?}",
                        pubsub.get_message(),
                    );
                }
                actives.insert(key, (conn, subscription));
            }
          }
          default => {
            return;
          }
      }
    }
}

fn consume_subscriptions(redis_client: &mut RedisClient, actives: &mut HashMap<String, (Connection, Subscription)>) {
    let mut removal = Vec::new();
    for (conn, subscription) in actives.values_mut().into_iter() {
        let mut pubsub = conn.as_pubsub();

        // TODO pubsub.get_message will block forever without timeout. The subscribe should be async
        pubsub.set_read_timeout(Some(Duration::from_secs(1)));

        println!(
            "bilibili subscription key: {}",
            subscription.redis_key()
        );
        while let Ok(msg) = pubsub.get_message() {
            println!(
                "bilibili 11111111111111111111111111111111111111111111111111111111: {}",
                subscription.redis_key()
            );
            match msg.get_payload() {
                Ok(payload) => {
                    if subscription.response().send(Ok(payload)).is_err() {
                        // we could not send value because context fired.
                        // skip all further messages on this stream, and queue for
                        // removal.
                        removal.push(subscription.redis_key());
                    }
                }
                Err(err) => error!("failed to decode subscription message: {:?}", msg),
            }
        }
    }

    for key in removal.iter() {
        actives.remove(key).expect("checked");
    }
}
