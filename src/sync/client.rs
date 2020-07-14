use crate::runtime::runenv::RunEnv;
use crate::runtime::runparams::RunParams;
use crate::sync::barrier::start_barrier_handler;
use crate::sync::types::Barrier;
use crossbeam_channel::{bounded, Sender};
use log::{debug, warn};
use redis::{
    Client as RedisClient, ConnectionLike, ErrorKind as RedisErrorKind, RedisError, RedisResult,
};
use std::env;

const REDIS_PAYLOAD_KEY: &str = "p";

#[derive(Debug, Clone)]
pub struct Client {
    runenv: RunEnv,
    redis_client: RedisClient,
    barrier_sender: Sender<Barrier>,
}

impl Client {
    pub fn new(runenv: RunEnv) -> Result<Self, String> {
        let mut redis_client = new_redis_client().map_err(|err| err.to_string())?;

        let (barrier_sender, barrier_receiver) = bounded(30);
        start_barrier_handler(&mut redis_client, barrier_receiver);

        Ok(Self {
            runenv,
            redis_client,
            barrier_sender,
        })
    }

    pub fn redis(&mut self) -> &mut RedisClient {
        &mut self.redis_client
    }

    pub fn barrier<S: ToString>(&self, state: S, target: u64) -> Result<(), String> {
        let rp = self.extractor(&self.runenv)?;

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

    // TODO FIXME
    fn extractor(&self, _runenv: &RunEnv) -> Result<RunParams, String> {
        Ok(RunParams::new(&Default::default()))
    }
}

fn new_redis_client() -> RedisResult<RedisClient> {
    let host = env::var("REDIS_HOST").expect("env[REDIS_HOST]");
    let mut port = 6379;
    if let Ok(port_str) = env::var("REDIS_PORT") {
        port = port_str.parse().expect("failed to parse REDIS_PORT");
    }

    debug!("trying redis host {} port {}", host, port);

    let redis_client = RedisClient::open(format!("{}:{}", host, port))?;
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
