use std::sync::{Arc, Mutex};

use log::info;
use rover::server::State;
use zmq::Message;

fn main() {
    rover::initialize_logging();
    info!("starting server");

    let state = Arc::new(Mutex::new(State::new()));

    let ctx = zmq::Context::new();
    let command_socket = ctx.socket(zmq::PUB).unwrap();
    let telemetry_socket = ctx.socket(zmq::SUB).unwrap();
    command_socket.bind("tcp://*:9311").unwrap();
    telemetry_socket.bind("tcp://*:9312").unwrap();
    telemetry_socket.set_subscribe(&[]).unwrap();

    while let Ok(message) = telemetry_socket.recv_msg(0) {
        info!("receved message: {:?}", message.as_str());
    }
}
