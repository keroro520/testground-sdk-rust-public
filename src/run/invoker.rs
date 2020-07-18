use crate::runtime::runenv::{current_run_env, RunEnv};
use crate::runtime::runenv_logger::Event;
use crate::runtime::runenv_logger::EventType::EventTypeStart;
use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};

// invoke runs the passed test-case and reports the result.
pub fn invoke<F>(f: F)
where
    F: Fn(RunEnv) -> Result<(), String>,
{
    let run_env = current_run_env().expect("invoke current_run_env");
    run_env.log("hello", Event::new(EventTypeStart));

    ::std::thread::sleep(::std::time::Duration::from_secs(6));

    // match f(run_env) {
    //     Ok(()) => {
    //         // TODO
    //     }
    //     Err(err) => {
    //         // TODO
    //     }
    // }
}
