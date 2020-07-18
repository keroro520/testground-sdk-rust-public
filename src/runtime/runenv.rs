use crate::runtime::runenv_logger::{Event, Logger};
use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};
use std::cell::Ref;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

/// RunEnv encapsulates the context for this test run.
#[derive(Clone)]
pub struct RunEnv {
    run_params: RunParams,

    logger: Arc<Mutex<Logger>>,
}

impl Deref for RunEnv {
    type Target = RunParams;
    fn deref(&self) -> &Self::Target {
        &self.run_params
    }
}

impl fmt::Debug for RunEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RunEnv").field(&self.run_params).finish()
    }
}

impl RunEnv {
    pub fn new(run_params: RunParams) -> Self {
        let logger = Arc::new(Mutex::new(Logger::init(&run_params)));
        Self { run_params, logger }
    }

    pub fn run_params(&self) -> &RunParams {
        &self.run_params
    }

    pub fn log(&self, msg: &str, event: Event) {
        if let Ok(mut logger) = self.logger.lock() {
            logger.log(self.run_params(), msg, event)
        }
    }

    pub fn current() -> Result<Self, String> {
        let env: HashMap<String, String> = env::vars_os()
            .into_iter()
            .map(|(key, val)| {
                (
                    key.to_string_lossy().to_string(),
                    val.to_string_lossy().to_string(),
                )
            })
            .collect();
        let run_params = RunParams::new(&env)?;
        Ok(RunEnv::new(run_params))
    }
}

pub fn current_run_env() -> Result<RunEnv, String> {
    RunEnv::current()
}
