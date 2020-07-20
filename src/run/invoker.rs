use crate::runtime::runenv::{current_run_env, RunEnv};
use crate::runtime::runparams::RunParams;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::panic::catch_unwind;
use std::sync::{Mutex, RwLock};

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
        Ok(file) => Mutex::new(file),
    };

    // Set a panic hook to catch the panic information.
    let run_env_clone = run_env.clone();
    ::std::panic::set_hook(Box::new(move |info| {
        let run_env = &run_env_clone;
        let backtrace = backtrace::Backtrace::new();
        let thread = ::std::thread::current();
        let name = thread.name().unwrap_or("unnamed");
        let location = info.location().unwrap(); // The current implementation always returns Some
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &*s,
                None => "Box<Any>",
            },
        };
        let err = format!(
            "thread '{}' panicked at '{}': {}:{}{:?}",
            name,
            msg,
            location.file(),
            location.line(),
            backtrace,
        );

        // Handle panics by recording them in the runenv output.
        run_env.record_crash(&err);

        // Developers expect panics to be recorded in run.err too.
        let mut errhandler = errfile.lock().unwrap();
        errhandler.write(err.as_bytes());
        errhandler.flush();
    }));

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
