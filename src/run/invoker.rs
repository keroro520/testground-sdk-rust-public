use crate::runtime::runenv::{current_run_env, RunEnv};
use serde::{Deserialize, Serialize};
use slog::{error, info};

#[derive(Debug)]
pub struct Foo {
    hello: String,
    world: f64,
}

// TODO remove `log`

// invoke runs the passed test-case and reports the result.
pub fn invoke<F>(f: F)
where
    F: Fn(RunEnv) -> Result<(), String>,
{
    let run_env = current_run_env().expect("invoke current_run_env");
    info!(run_env.l(), "start-raw");
    let foo = Foo {
        hello: "hahhaah".to_string(),
        world: 1.2,
    };
    info!(run_env.l(), "{:?}", foo);

    match f(run_env) {
        Ok(()) => {
            // TODO
        }
        Err(err) => {
            // TODO
        }
    }
}
