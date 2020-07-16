use crate::runtime::runenv::RunEnv;
use crate::runtime::runparams::RunParams;
use crate::sync::barrier::start_barrier_handler;
use crate::sync::subscription::start_subscription_handler;
use crate::sync::types::{Barrier, Payload, State, Subscription, Topic};
use crossbeam_channel::{bounded, Receiver, Sender};
use log::{debug, warn};
use redis::{
    Client as RedisClient, Commands, ConnectionLike, ErrorKind as RedisErrorKind, RedisError,
    RedisResult,
};
use std::env;
use std::thread::spawn;

pub const ENV_REDIS_HOST: &str = "REDIS_HOST";
pub const ENV_REDIS_PORT: &str = "REDIS_PORT";
pub const REDIS_PAYLOAD_KEY: &str = "p";

#[derive(Debug, Clone)]
pub struct Client {
    pub runenv: RunEnv, // TODO FIXME remove `pub`
    redis_client: RedisClient,
    barrier_sender: Sender<Barrier>,
    subscription_sender: Sender<Subscription>,
}

impl Client {
    pub fn new(runenv: RunEnv) -> Result<Self, String> {
        let mut redis_client = new_redis_client().map_err(|err| err.to_string())?;
        let barrier_sender = start_barrier_handler(redis_client.clone());
        let subscription_sender = start_subscription_handler(redis_client.clone());
        Ok(Self {
            runenv,
            redis_client,
            barrier_sender,
            subscription_sender,
        })
    }

    pub fn redis(&mut self) -> &mut RedisClient {
        &mut self.redis_client
    }

    pub fn barrier<S: ToString>(&self, state: S, target: u64) -> Result<(), String> {
        let rp = self.runenv.run_params();

        // a barrier with target zero is satisfied immediately; log a warning as
        // this is probably programmer error.
        if target == 0 {
            warn!(
                "requested a barrier with target zero; satisfying immediately, state: {}",
                state.to_string()
            );
            return Ok(());
        }

        let (barrier, ch) = Barrier::new(&rp, state, target);
        self.barrier_sender
            .send(barrier)
            .map_err(|err| err.to_string())?;
        match ch.recv() {
            Ok(result) => result,
            Err(err) => Err(err.to_string()),
        }
    }

    // SignalEntry increments the state counter by one, returning the value of the
    // new value of the counter, or an error if the operation fails.
    pub fn signal_entry(&mut self, state: &State) -> Result<u64, String> {
        let rp = self.runenv.run_params();

        // Increment a counter on the state key
        let key = state.redis_key(&rp);
        let mut conn = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        conn.incr(key, 1).map_err(|err| err.to_string())
    }

    /// publish publishes an item on the supplied topic. The payload type must match
    /// the payload type on the Topic; otherwise Publish will error.
    pub fn publish(&mut self, topic: &Topic, payload: Payload) -> Result<u64, String> {
        let rp = self.runenv.run_params();
        let redis_key = topic.redis_key(rp);
        if !topic.validate_payload(payload) {
            return Err("invalid payload".to_string());
        }

        let mut conn = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        conn.publish(redis_key, payload)
            .map_err(|err| err.to_string())
    }

    /// subscribe subscribes to a topic, consuming ordered, typed elements from
    /// index 0, and sending them to channel ch.
    ///
    /// The supplied channel must be buffered, and its type must be a value or
    /// pointer type matching the topic type. If these conditions are unmet, this
    /// method will error immediately.
    ///
    /// The caller must consume from this channel promptly; failure to do so will
    /// backpressure the DefaultClient's subscription event loop.
    pub fn subscribe(&self, topic: Topic) -> Result<Receiver<Result<Payload, String>>, String> {
        let rp = self.runenv.run_params();
        let redis_key = topic.redis_key(rp);
        let (subscription, sub_response_receiver) = Subscription::new(topic, redis_key);
        self.subscription_sender
            .send(subscription)
            .map_err(|err| err.to_string())?;
        Ok(sub_response_receiver)
    }
}

pub(crate) fn new_redis_client() -> RedisResult<RedisClient> {
    let host = env::var(ENV_REDIS_HOST).unwrap_or_else(|_| "localhost".to_string());
    let mut port = 6379;
    if let Ok(port_str) = env::var(ENV_REDIS_PORT) {
        port = port_str.parse().expect("failed to parse REDIS_PORT");
    }

    debug!("trying redis host {} port {}", host, port);

    let redis_client = RedisClient::open(format!("redis://{}:{}", host, port))?;
    let mut conn = redis_client.get_connection()?;
    if !conn.check_connection() {
        return Err(RedisError::from((
            RedisErrorKind::ClientError,
            "failed to ping redis",
        )));
    }

    debug!("redis ping OK");

    Ok(redis_client)
}
