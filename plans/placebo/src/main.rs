use std::thread::sleep;
use std::time::Duration;
use testground::run::invoker::invoke;
use testground::runtime::runenv::RunEnv;

fn main() {
    invoke(run_);
}

fn run_(runenv: RunEnv) -> Result<(), String> {
    match runenv.test_case.as_str() {
        "ok" => return Ok(()),
        "panic" => panic!("this is an intentional panic".to_string()),
        "stall" => {
            sleep(Duration::from_secs(24 * 60 * 60 /* 24h */));
            return Ok(());
        }
        _ => return Err("aborting".to_string()),
    }
}
