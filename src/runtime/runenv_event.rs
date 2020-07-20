use crate::runtime::runenv::RunEnv;
use serde::{Deserialize, Serialize};

pub type EventType = str;
pub type EventOutcome = str;

pub const EVENT_TYPE_START: &EventType = "start";
pub const EVENT_TYPE_FINISH: &EventType = "finish";
pub const EVENT_TYPE_MESSAGE: &EventType = "message";

pub const EVENT_OUTCOME_OK: &EventOutcome = "ok";
pub const EVENT_OUTCOME_FAILED: &EventOutcome = "failed";
pub const EVENT_OUTCOME_CRASHED: &EventOutcome = "crashed";

// TODO omitempty

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Event {
    #[serde(rename = "type")]
    pub type_: String,
    pub outcome: String,
    pub error: String,
    pub stacktrace: String,
    pub message: String,
}

impl Event {
    pub fn new(type_: &EventType) -> Self {
        Self {
            type_: type_.to_string(),
            ..Default::default()
        }
    }
}

impl RunEnv {
    // record_message records an informational message.
    pub fn record_message(&self, message: &str) {
        let event = Event {
            type_: EVENT_TYPE_MESSAGE.to_string(),
            message: message.to_string(),
            ..Default::default()
        };
        self.log("", event)
    }

    pub fn record_start(&self) {
        let event = Event {
            type_: EVENT_TYPE_START.to_string(),
            ..Default::default()
        };
        self.log("", event)
    }

    // record_success records that the calling instance succeeded.
    pub fn record_success(&self) {
        let event = Event {
            type_: EVENT_TYPE_FINISH.to_string(),
            outcome: EVENT_OUTCOME_OK.to_string(),
            ..Default::default()
        };
        self.log("", event)
    }

    // RecordFailure records that the calling instance failed with the supplied
    // error.
    pub fn record_failure<S: ToString>(&self, err: &S) {
        let event = Event {
            type_: EVENT_TYPE_FINISH.to_string(),
            outcome: EVENT_OUTCOME_FAILED.to_string(),
            error: err.to_string(),
            ..Default::default()
        };
        self.log("", event)
    }

    // record_crash records that the calling instance crashed/panicked with the
    // supplied error.
    pub fn record_crash<S: ToString>(&self, err: &S) {
        let event = Event {
            type_: EVENT_TYPE_FINISH.to_string(),
            outcome: EVENT_OUTCOME_CRASHED.to_string(),
            error: err.to_string(),
            stacktrace: "".to_string(), // TODO
            ..Default::default()
        };
        self.log("", event)
    }
}
