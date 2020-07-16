use crate::runtime::test_utils::random_test_run_env;
use crate::sync::client::Client;
use crate::sync::types::{Payload, Topic};
use crossbeam_channel::Receiver;
use redis::Commands;
use std::thread::{sleep, spawn};
use std::time::Duration;

#[test]
fn test_subscribe_after_all_published() {
    let iterations: u64 = 1000;
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let payloads = (0..iterations).collect::<Vec<_>>();
    let topic = Topic::new("pandemic");

    produce(&mut client, &topic, &payloads);

    let sub_response_receiver = client.subscribe(topic).expect("subscribe");
    consume_ordered(sub_response_receiver, &payloads);
}

#[test]
fn test_subscribe_first_concurrent_writes() {
    let iterations: u64 = 1000;
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let payloads = (0..iterations).collect::<Vec<_>>();
    let topic = Topic::new("virus");

    let sub_response_receiver = client.subscribe(topic.clone()).expect("subscribe");

    let mut client_clone = client.clone();
    let topic_clone = topic.clone();
    let payloads_clone = payloads.clone();
    spawn(move || {
        produce(&mut client_clone, &topic_clone, &payloads_clone);
    });

    consume_ordered(sub_response_receiver, &payloads);
}

#[test]
fn test_subscribe_duplicate_topic() {
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let topic = Topic::new("immune");

    let _ = client.subscribe(topic.clone()).expect("subscribe");
    let sub_response_receiver = client.subscribe(topic.clone()).expect("subscribe");

    assert!(
        sub_response_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap()
            .is_err(),
        "duplicated subscription"
    );
}

fn produce(client: &mut Client, topic: &Topic, payloads: &Vec<Payload>) {
    let rp = client.runenv.run_params();
    let redis_key = topic.redis_key(rp);
    let mut conn = client.redis().get_connection().unwrap();

    for (i, payload) in payloads.iter().enumerate() {
        let seq_id: u64 = client.publish(topic, *payload).unwrap();
        assert_eq!(seq_id, 1 + i as u64);
    }
}

fn consume_ordered(
    sub_response_receiver: Receiver<Result<Payload, String>>,
    payloads: &Vec<Payload>,
) {
    for (i, payload) in payloads.iter().enumerate() {
        let response = sub_response_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("subscription message")
            .expect("ok");
        assert_eq!(
            response, *payload,
            "expected value {}, got {} in position {}",
            payload, response, i
        );
    }
}
