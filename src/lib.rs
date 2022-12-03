pub mod server;
pub mod onboard;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    Ok,
    Problem(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Telemetry {
    pub timestamp: DateTime<Utc>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    pub motor: f32,
    pub steering: f32,
}

pub fn initialize_logging() {
    env_logger::init();
}
