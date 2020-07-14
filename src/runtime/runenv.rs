use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ops::Deref;

// RunEnv encapsulates the context for this test run.
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
}

pub fn current_run_env() -> RunEnv {
    let vars_os = env::vars_os();
    let mut env = HashMap::new();
    for (key, val) in vars_os {
        env.insert(
            key.to_string_lossy().to_string(),
            val.to_string_lossy().to_string(),
        );
    }
    let run_params = RunParams::new(&env);
    RunEnv::new(run_params)
}
