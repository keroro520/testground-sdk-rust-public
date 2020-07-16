use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ops::Deref;

/// RunEnv encapsulates the context for this test run.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RunEnv {
    run_params: RunParams,
}

impl Deref for RunEnv {
    type Target = RunParams;
    fn deref(&self) -> &Self::Target {
        &self.run_params
    }
}

impl RunEnv {
    pub fn new(run_params: RunParams) -> Self {
        Self { run_params }
    }

    pub fn run_params(&self) -> &RunParams {
        &self.run_params
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
