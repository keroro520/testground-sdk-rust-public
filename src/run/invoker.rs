use crate::runtime::runenv::{current_run_env, RunEnv};
use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};

// invoke runs the passed test-case and reports the result.
pub fn invoke<F>(f: F)
where
    F: Fn(RunEnv) -> Result<(), String>,
{
    wait_network_initialize();

    let run_env = current_run_env().expect("current_run_env");
    run_env.record_start();

    match f(run_env.clone()) {
        Ok(()) => run_env.record_success(),
        Err(err) => run_env.record_failure(&err),
    }

    run_env.record_message("io closed");
}

// TODO  wait_network_initialize
fn wait_network_initialize() {
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
}
