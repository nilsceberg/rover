use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use log::{debug, info, warn};
use zmq::Message;

pub mod vehicle;

use vehicle::Vehicle;

use crate::{Command, Status, Telemetry};

pub struct OnboardConfig {
    pub telemetry_endpoint: String,
    pub command_endpoint: String,
}

struct State {
    motor: f32,
    steering: f32,
    last_contact: Option<DateTime<Utc>>,
}

impl State {
    pub fn new() -> State {
        State {
            last_contact: None,
            motor: 0.0,
            steering: 0.0,
        }
    }
}

fn listen_for_commands(socket: zmq::Socket, state: Arc<Mutex<State>>) {
    while let Ok(message) = socket.recv_bytes(0) {
        let command = serde_json::from_slice::<Command>(&message).unwrap();
        let mut state = state.lock().unwrap();
        state.motor = command.motor;
        state.steering = command.steering;
        state.last_contact = Some(Utc::now());
    }
}

pub fn run(config: OnboardConfig, mut vehicle: impl Vehicle) {
    info!("starting onboard systems");

    // Open ZeroMQ sockets
    let ctx = zmq::Context::new();
    let command_socket = ctx.socket(zmq::SUB).unwrap();
    command_socket.set_subscribe(&[]).unwrap();
    let telemetry_socket = ctx.socket(zmq::PUB).unwrap();
    command_socket.connect(&config.command_endpoint).unwrap();
    telemetry_socket
        .connect(&config.telemetry_endpoint)
        .unwrap();

    let state = Arc::new(Mutex::new(State::new()));

    // Start command listener
    let _command_thread_handle = {
        let state = state.clone();
        std::thread::spawn(move || listen_for_commands(command_socket, state))
    };

    // Some local state
    let mut last_telemetry_time = Utc::now();
    let mut command_is_fresh = false;

    // Run vehicle control loop and send telemetry
    loop {
        let now = Utc::now();

        {
            let freshness_timeout = chrono::Duration::milliseconds(500);
            let state = state.lock().unwrap();
            // Check that input is fresh enough
            if let Some(last_contact) = state.last_contact {
                if (now - last_contact) > freshness_timeout {
                    if command_is_fresh {
                        warn!("no command received; emergency stop");
                    }
                    command_is_fresh = false;
                    vehicle.set_motor(0.0);
                    vehicle.set_turn(0.0);
                } else {
                    vehicle.set_motor(state.motor);
                    vehicle.set_turn(state.steering);
                }
            }
        }

        let time_since_last_telemetry = now - last_telemetry_time;
        let it_is_time_to_send_telemetry = time_since_last_telemetry >= Duration::seconds(1);
        if it_is_time_to_send_telemetry {
            last_telemetry_time = now;

            debug!("preparing telemetry...");
            let mut telemetry = Telemetry {
                status: Status::Ok,
                timestamp: chrono::Utc::now(),
            };

            vehicle.update_telemetry(&mut telemetry);

            telemetry_socket
                .send(serde_json::to_vec(&telemetry).unwrap(), 0)
                .unwrap();
            debug!("telemetry sent");
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
