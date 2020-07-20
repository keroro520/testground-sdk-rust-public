use crate::runtime::runenv::{current_run_env, RunEnv};
use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::panic::catch_unwind;

// invoke runs the passed test-case and reports the result.
pub fn invoke<F>(f: F)
where
    F: FnOnce(RunEnv) -> Result<(), String> + std::panic::UnwindSafe,
{
    wait_network_initialize();

    let run_env = current_run_env().expect("current_run_env");

    run_env.record_start();

    // TODO redirect stderr into errfile
    let mut errfile = match run_env.create_raw_asset("run.err") {
        Err(err) => {
            run_env.record_crash(&err);
            return;
        }
        Ok(file) => file,
    };

    let run_env_clone = run_env.clone();
    let recover = catch_unwind(move || {
        let run_env = run_env_clone;
        match f(run_env.clone()) {
            Ok(()) => run_env.record_success(),
            Err(err) => run_env.record_failure(&err),
        }
    });
    if let Err(err) = recover {
        // Handle panics by recording them in the runenv output.
        run_env.record_crash(&format!("{:?}", err));

        // Developers expect panics to be recorded in run.err too.
        errfile.write(&format!("{:?}", err).as_bytes());
        errfile.flush();
    }

    run_env.record_message("io closed");
}

// TODO  wait_network_initialize
fn wait_network_initialize() {
    ::std::thread::sleep(::std::time::Duration::from_secs(3));
}
