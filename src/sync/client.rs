use redis::{Client as RedisClient, ConnectionLike, RedisResult, RedisError, ErrorKind as RedisErrorKind};
use log::{info, error, debug};
use crate::types::{RunEnv, State, Barrier};
use crate::barrier::start_barrier_handler;
use crossbeam_channel::{bounded, Sender};

const REDIS_PAYLOAD_KEY: &str = "p";
const ENV_REDIS_HOST: &str = "REDIS_HOST";
const ENV_REDIS_PORT: &str = "REDIS_PORT";

// TODO Error management

#[derive(Debug, Clone)]
pub struct Client {
    runenv: RunEnv,
    redis_client: RedisClient,
    barrier_sender: Sender<Barrier>,
}

impl Client {
    pub fn new(runenv: RunEnv) -> Result<Self, String> {
        let redis_client = new_redis_client()
            .map_err(|err| err.to_string())?;

        let (barrier_sender, barrier_receiver) = bounded(30);
        start_barrier_handler(redis_client.clone(), barrier_receiver);

        Ok( Self { runenv, redis_client, barrier_sender } )
    }

    pub fn redis(&mut self) -> &mut RedisClient {
        &mut self.redis_client
    }

    pub fn barrier<S: ToString>(&self, state: S, target: u64) -> Result<(), String> {
        let rp = self.extractor(&self.runenv)?;

        // a barrier with target zero is satisfied immediately; log a warning as
        // this is probably programmer error.
        if target == 0 {
            warn!("requested a barrier with target zero; satisfying immediately, state: {:?}", state);
            return Ok(());
        }

        let (barrier, ch) = Barrier::new(&rp, state, target);
        self.barrier_sender.send(barrier).map_err(|err| err.to_string())?;
        match ch.recv() {
            Ok(result) => result,
            Err(err) => Err(err.to_string()),
        }
    }
}

fn new_redis_client() -> RedisResult<RedisClient> {
    let host = env!(ENV_REDIS_HOST);
    let port: u64 = match option_env!(ENV_REDIS_PORT) {
        Some(port_str) =>
            port_str.parse()
                .map_err(|err| format!("failed to parse {}: {}", ENV_REDIS_PORT, port_str))?,
        None => 6379,
    };

    debug!("trying redis host {} port {}", host, port);

    let redis_client =  RedisClient::open(format!("{}:{}", host, port))?;
    let mut conn = redis_client.get_connection()
        .map_err(|err| err.to_string())?;
    if !conn.check_connection() {
        return Err(RedisError::from((RedisErrorKind::ClientError, "failed to ping redis")));
    }

    debug!("redis ping OK");

    Ok(redis_client)
}