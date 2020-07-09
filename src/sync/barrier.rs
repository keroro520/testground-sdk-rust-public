use redis::Client as RedisClient;
use crossbeam_channel::{bounded, Receiver, select, tick, never};
use crate::types::Barrier;
use std::time::Duration;
use std::collections::HashSet;
use log::warn;
use redis::Commands;

const BARRIER_TICK_DURATION: Duration = Duration::from_secs(1);

pub(crate) fn start_barrier_handler(mut redis_client: RedisClient, receiver: Receiver<Barrier>) {
    let mut pending: Vec<Barrier> = Vec::new();
    let mut ticker = never();
    loop {
        select! {
            recv(receiver) -> barrier => {
              pending.push(barrier);

              if pending.len() == 1 {
                ticker = tick(BARRIER_TICK_DURATION);
              }
            }
            recv(ticker) -> _ => {
              // check barriers periodically

              if pending.is_empty() {
                ticker = never();
              }
            }
        }

        let conn = match redis_client.get_connection() {
            Ok(conn) => {
                conn
            }
            Err(err) => {
                warn!("failed to get connection; iteration skipped, error {:?}", err);
                continue
            }
        };
        let keys = pending.iter().map(|barrier| barrier.key()).collect();
        let vals: Vec<String> = match redis::cmd("MGET").arg(&keys).query(&conn) {
            Ok(vals) => {
                // TODO vals 是什么类型？
                vals
            }
            Err(err) => {
                warn!("failed while getting barriers; iteration skipped, error {:?}", err);
                continue
            }
        };

        let mut hits = Vec::new();
        for (i, (barrier, val)) in pending.iter().zip(vals.iter()).enumerate() {
            // nobody else has INCR the barrier yet; skip.
            if val.is_empty() {
                continue
            }

            let curr = match val.parse::<u64>() {
                Ok(curr) => curr,
                Err(err) => {
                    warn!("failed to parse barrier value, error: {:?}, value: {}, key: {}", err, val, barrier.key());
                    continue
                }
            };

            // Has the barrier been hit?
            if curr >= barrier.target() {
                debug!("barrier was hit; informing waiters, key: {}, target: {}, curr: {}", barrier.key(), barrier.target(), curr);
                if let Err(err) = barrier.ch().send(Ok(())) {
                    warn!("barrier waiter is already closed, key: {}, target: {}, curr: {}");
                }
                hits.push(i)
            } else {
                debug!("barrier still unsatisfied, key: {}, target: {}, curr: {}", barrier.key(), barrier.target(), curr);
            }
        }

        for (prefix, i) in hits.into_iter().enumerate() {
            pending.remove(i - prefix);
        }
    }
}
