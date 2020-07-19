use crate::runtime::runenv::RunEnv;
use crate::runtime::runparams::RunParams;
use serde_json::json;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;

use crossbeam_channel::{bounded, Sender};
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::thread::{spawn, JoinHandle};

pub type EventType = str;
pub type EventOutcome = str;

pub const EVENT_TYPE_START: &EventType = "start";
pub const EVENT_TYPE_FINISH: &EventType = "finish";
pub const EVENT_TYPE_MESSAGE: &EventType = "message";

pub const EVENT_OUTCOME_OK: &EventOutcome = "ok";
pub const EVENT_OUTCOME_FAILED: &EventOutcome = "failed";
pub const EVENT_OUTCOME_CRASHED: &EventOutcome = "crashed";

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Event {
    #[serde(rename = "type")]
    pub type_: String,
    pub outcome: String,
    pub error: String,
    pub stacktrace: String,
    pub message: String,
}

pub struct Logger {
    file: File,
}

impl Event {
    pub fn new(type_: &EventType) -> Self {
        Self {
            type_: type_.to_string(),
            ..Default::default()
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = self.file.flush();
    }
}

impl Logger {
    pub fn init(rp: &RunParams) -> Self {
        assert!(!rp.test_outputs_path.is_empty());
        let log_path = Path::new(&rp.test_outputs_path).join("run.out");
        let log_file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .create_new(true)
            .open(&log_path)
            .expect(&format!("failed to open {}", log_path.to_string_lossy()));
        Self { file: log_file }
    }

    pub fn log(&mut self, rp: &RunParams, msg: &str, event: Event) {
        // Example {"ts":1595070350599936400,"msg":"","group_id":"single","run_id":"25cf14535bc5","event":{"type":"message","message":"io closed"}}
        let log = json!({
            // TODO event ts
            "ts": 1595070350599936400u64,
            "msg": msg,
            "group_id": rp.test_group_id,
            "run_id": rp.test_run,
            "event": event,
        });
        let str = log.to_string();
        if self.file.write(str.as_bytes()).is_ok() {
            println!("{}", str);
        }
    }
}
