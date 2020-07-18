use crate::runtime::runenv::RunEnv;
use crate::runtime::runparams::RunParams;
use slog::{info, o, Drain, Logger, OwnedKV, OwnedKVList};
use slog_bunyan;
use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

pub(crate) fn init_logger(rp: &RunParams) -> Logger {
    // TODO LOG_LEVEL
    // let log_level = env::var("LOG_LEVEL").unwrap_or("debug".to_string());

    assert!(!rp.test_outputs_path.is_empty());
    let log_path = Path::new(&rp.test_outputs_path).join("run.out");
    let log_file = OpenOptions::new()
        .append(true)
        .truncate(true)
        .open(&log_path)
        .expect(&format!("failed to open {}", log_path.to_string_lossy()));

    // TODO timestamp format
    let drain = slog_bunyan::with_name("", log_file).build();
    let logger = Logger::root(
        Mutex::new(drain).fuse(),
        o!("run_id" => rp.test_run.clone(), "group_id" => rp.test_group_id.clone()),
    );

    logger
}
