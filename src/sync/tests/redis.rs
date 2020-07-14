use crate::sync::client::{new_redis_client, ENV_REDIS_HOST};
use std::env;

#[test]
fn test_redis_host() {
    let real_redis_host = env::var(ENV_REDIS_HOST);
    let real_redis_host_clone = real_redis_host.clone();
    defer::defer(|| match real_redis_host_clone {
        Ok(host) => env::set_var(ENV_REDIS_HOST, host),
        Err(_) => env::set_var(ENV_REDIS_HOST, ""),
    });

    env::set_var(ENV_REDIS_HOST, "redis-does-not-exist.example.com");
    let result = new_redis_client();
    assert!(result.is_err(), "should not have found redis host");

    if real_redis_host.is_err() || real_redis_host == Ok("".to_string()) {
        env::set_var(ENV_REDIS_HOST, "localhost");
    } else if let Ok(host) = real_redis_host {
        env::set_var(ENV_REDIS_HOST, host);
    };
    let result = new_redis_client();
    assert!(
        result.is_ok(),
        "expected to establish connection to redis, but failed with: {:?}",
        result.unwrap_err()
    );
}
