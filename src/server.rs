use chrono::{DateTime, Utc};
use serde::{Serialize};

use crate::Telemetry;

#[derive(Serialize, Clone)]
pub struct State {
    pub last_heartbeat_time: Option<DateTime<Utc>>,
    pub telemetry: Option<Telemetry>,
}

impl State {
    pub fn new() -> State {
        State {
            telemetry: None,
            last_heartbeat_time: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        let timeout: chrono::Duration = chrono::Duration::seconds(5);
        let now = chrono::Utc::now();
        match self.last_heartbeat_time {
            None => false,
            Some(time) => (now - time) >= timeout,
        }
    }
}
