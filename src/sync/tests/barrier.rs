use crate::runtime::test_utils::random_test_run_env;
use crate::sync::client::Client;
use crate::sync::types::State;
use crossbeam_channel::bounded;
use std::thread::spawn;
use std::time::Duration;

// TODO ensure redis running when setup tests

#[test]
fn test_barrier() {
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let state = State::new("yoda");

    let (notify_sender, notify_receiver) = bounded(1);
    let state_clone = state.clone();
    let client_clone = client.clone();
    spawn(move || {
        notify_sender
            .send(client_clone.barrier(state_clone, 10))
            .unwrap();
    });

    for i in 1..=10u64 {
        assert_eq!(client.signal_entry(&state), Ok(i));
        if i == 9 {
            assert!(
                notify_receiver
                    .recv_timeout(Duration::from_secs(2))
                    .is_err(),
                "it should be timeout since the barrier is still works"
            );
        } else if i == 10 {
            assert!(
                notify_receiver.recv_timeout(Duration::from_secs(2)).is_ok(),
                "we should receive the barrier message"
            );
        }
    }
}

#[test]
fn test_barrier_beyond_target() {
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let state = State::new("yoda");

    for i in 1..=20u64 {
        assert_eq!(client.signal_entry(&state), Ok(i));
    }

    assert!(client.barrier(state, 10).is_ok());
}

#[test]
fn test_barrier_zero() {
    let (runenv, _output_dir) = random_test_run_env();
    let mut client = Client::new(runenv).expect("new client");
    let state = State::new("yoda");
    assert!(client.barrier(state, 0).is_ok());
}
