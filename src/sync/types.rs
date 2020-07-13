use crossbeam_channel::{Receiver, bounded, Sender};
use runtime::run_params::RunParams;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct State {
    state: String,
    redis_key: String,
}

impl State {
    pub(crate) fn new<S: ToString>(rp: &RunParams, state: S) -> Self {
        let state = state.to_string();
        let redis_key = format!("run:{}:plan:{}:case:{}:states:{}", rp.test_run, rp.test_plan, rp.test_case, state);
        Self {state, redis_key}
    }

    pub(crate) fn state(&self) -> &str {
        &self.state
    }

    pub(crate) fn redis_key(&self) -> &str {
        &self.redis_key
    }
}

pub struct Barrier {
    ch: Sender<Result<(), String>>,

    state: State,
    target: u64,
}

impl Barrier {
    pub fn new<S: ToString>(rp: &RunParams, state: S, target: u64) -> (Self, Receiver<Result<(), String>>) {
        let state = State::new(rp, state);
        let (sender, receiver) = bounded(1);
        let barrier = Self {
            ch: sender,
            state,
            target,
        };
        (barrier, receiver)
    }

    pub fn key(&self) -> &str {
        &self.state.redis_key()
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
