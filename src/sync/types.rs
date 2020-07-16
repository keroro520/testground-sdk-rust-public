use crate::runtime::runparams::RunParams;
use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::fmt;

/// State represents a state in a distributed state machine, identified by a
/// unique string within the test case.
#[derive(Debug, Clone)]
pub struct State {
    name: String,
}

/// Barrier represents a barrier over a State. A Barrier is a synchronisation
/// checkpoint that will fire once the `target` number of entries on that state
/// have been registered.
pub struct Barrier {
    response: Sender<Result<(), String>>,

    state: State,
    redis_key: String,
    target: u64,
}

/// Topic represents a meeting place for test instances to exchange arbitrary
/// data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    name: String,
}

// TODO The payload can be more general
pub type Payload = u64;

// Subscription represents a receive channel for data being published in a
// Topic.
pub struct Subscription {
    response: Sender<Result<Payload, String>>,

    topic: Topic,
    redis_key: String,
}

impl ToString for State {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

// Barrier represents a barrier over a State. A Barrier is a synchronisation
// checkpoint that will fire once the `target` number of entries on that state
// have been registered.
impl State {
    pub(crate) fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    // The original state name
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    // The Redis key for this State, contextualized to a set of RunParams.
    pub(crate) fn redis_key(&self, rp: &RunParams) -> String {
        format!(
            "run:{}:plan:{}:case:{}:states:{}",
            rp.test_run, rp.test_plan, rp.test_case, self.name
        )
    }
}

impl Barrier {
    pub fn new<S: ToString>(
        rp: &RunParams,
        state: S,
        target: u64,
    ) -> (Self, Receiver<Result<(), String>>) {
        let state = State::new(state);
        let redis_key = state.redis_key(rp);
        let (sender, receiver) = bounded(1);
        let barrier = Self {
            response: sender,
            state,
            redis_key,
            target,
        };
        (barrier, receiver)
    }

    pub fn key(&self) -> String {
        self.redis_key.clone() // TODO
    }

    pub fn target(&self) -> u64 {
        self.target
    }

    pub fn response(&self) -> &Sender<Result<(), String>> {
        &self.response
    }
}

impl fmt::Debug for Barrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Barrier")
            .field(&self.state)
            .field(&self.target)
            .finish()
    }
}

impl Topic {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    // Returns the key for this Topic, contextualized to a set of RunParams.
    pub fn redis_key(&self, rp: &RunParams) -> String {
        format!(
            "run:{}:plan:{}:case:{}:topics:{}",
            rp.test_run, rp.test_plan, rp.test_case, self.name
        )
    }

    pub fn validate_payload(&self, _payload: Payload) -> bool {
        true
    }

    pub fn decode_payload(&self) {}
}

impl Subscription {
    pub fn new(topic: Topic, redis_key: String) -> (Self, Receiver<Result<Payload, String>>) {
        let (sender, receiver) = bounded(1000);
        (
            Self {
                response: sender,
                topic,
                redis_key,
            },
            receiver,
        )
    }

    pub fn response(&self) -> &Sender<Result<Payload, String>> {
        &self.response
    }

    pub fn redis_key(&self) -> String {
        self.redis_key.clone()
    }
}

impl fmt::Debug for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Subscription")
            .field(&self.topic)
            .field(&self.redis_key)
            .finish()
    }
}
