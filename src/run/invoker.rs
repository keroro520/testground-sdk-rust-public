use crate::runtime::runenv::{current_run_env, RunEnv};
use log::{error, info};

// invoke runs the passed test-case and reports the result.
pub fn invoke<F>(f: F)
where
    F: Fn(RunEnv) -> Result<(), String>,
{
    let run_env = current_run_env().expect("invoke current_run_env");
    match f(run_env) {
        Ok(()) => {
            // TODO
            info!("success");
        }
        Err(err) => {
            // TODO
            error!("failed: {:?}", err);
        }
    }
}
