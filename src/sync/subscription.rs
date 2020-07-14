use crossbeam_channel::{bounded, select, Receiver, Sender};
use log::warn;
use redis::{Client as RedisClient, Commands};
use std::collections::HashMap;

// pub(crate) fn start_subscription_handler(redis_client: &mut RedisClient, add_sub_receiver: &Receiver<AddSubscription>) {
//     let mut actives = HashMap::new();
//     let (rm_subs_sender, rm_subs_receiver) = bounded(1);
//
//     let consumer = SubscriptionConsumer::new();
//     loop {
//         manage_subscriptions(&mut actives, add_sub_receiver, &rm_subs_receiver);
//         if !actives.is_empty() {
//             consume_subscriptions(redis_client, &mut actives);
//         }
//     }
// }
//
// fn manage_subscriptions(
//     actives: &mut HashMap<String, Subscription>,
//     add_sub_receiver: &Sender<AddSubscription>,
//     rm_subs_receiver: &Sender<RmSubscription>) {
//     select! {
//       recv(add_sub_receiver) -> add_subsciption => {
//         if actives.contains_key(add_subscription.redis_key()) {
//           add_subscription.response(Err("failed to add duplicate subscription".to_string()));
//           continue
//         }
//
//         let AddSubscription { subscription, ch } = add_subscription;
//         actives.insert(subscription.redis_key(), subscription);
//         // TODO 如何把结果返回回去？
//       }
//       recv(rm_subs_receiver) -> rm_subscriptions => {
//         // TODO
//       }
//       default => {
//         // pass, do nothing
//       }
//     }
// }
//
// fn consume_subscriptions(redis_client: &mut RedisClient, actives: &mut HashMap<String, Subscription>) {
//     let mut conn = match redis_client.get_connection() {
//         Ok(conn) => {
//             conn
//         }
//         Err(err) => {
//             warn!("failed to get connection; iteration skipped, error {:?}", err);
//             return;
//         }
//     };
//     let mut pubsub = conn.as_pubsub();
//     // TODO
// }
