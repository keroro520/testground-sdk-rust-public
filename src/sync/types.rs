use crate::runtime::runparams::RunParams;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::fmt;

/// State represents a state in a distributed state machine, identified by a
/// unique string within the test case.
#[derive(Debug, Clone)]
pub(crate) struct State {
    runparams: RunParams,
    name: String,
}

/// Barrier represents a barrier over a State. A Barrier is a synchronisation
/// checkpoint that will fire once the `target` number of entries on that state
/// have been registered.
pub struct Barrier {
    ch: Sender<Result<(), String>>,

    state: State,
    target: u64,
}

/// Topic represents a meeting place for test instances to exchange arbitrary
/// data.
pub struct Topic {
    runparams: RunParams,
    name: String,
    typ: String, // TODO enum it
}

// Barrier represents a barrier over a State. A Barrier is a synchronisation
// checkpoint that will fire once the `target` number of entries on that state
// have been registered.
impl State {
    pub(crate) fn new<S: ToString>(runparams: RunParams, name: S) -> Self {
        let name = name.to_string();
        Self { runparams, name }
    }

    // The original state name
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    // The Redis key for this State, contextualized to a set of RunParams.
    pub(crate) fn redis_key(&self) -> String {
        let rp = &self.runparams;
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
        let state = State::new(rp.clone(), state);
        let (sender, receiver) = bounded(1);
        let barrier = Self {
            ch: sender,
            state,
            target,
        };
        (barrier, receiver)
    }

    pub fn key(&self) -> String {
        self.state.redis_key()
    }

    pub fn target(&self) -> u64 {
        self.target
    }

    pub fn ch(&self) -> &Sender<Result<(), String>> {
        &self.ch
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
    pub fn new<S: ToString>(runparams: RunParams, name: S, typ: S) -> Self {
        Self {
            runparams,
            name: name.to_string(),
            typ: typ.to_string(),
        }
    }

    // Returns the key for this Topic, contextualized to a set of RunParams.
    pub fn redis_key(&self) -> String {
        let rp = &self.runparams;
        format!(
            "run:{}:plan:{}:case:{}:topics:{}",
            rp.test_run, rp.test_plan, rp.test_case, self.name
        )
    }

    // TODO validate_payload
    fn validate_payload(&self) -> bool {
        true
    }

    // TODO decode_payload
    fn decode_payload(&self) {}
}
