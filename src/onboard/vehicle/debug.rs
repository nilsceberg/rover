use log::debug;

use crate::{Telemetry, Status};

use super::Vehicle;

pub struct DebugVehicle {
}

impl Vehicle for DebugVehicle {
    fn set_motor(&mut self, speed: f32) {
        debug!("set motor speed to {}", speed);
    }

    fn set_turn(&mut self, turn: f32) {
        debug!("set turn to {}", turn);
    }

    fn update_telemetry(&self, telemetry: &mut Telemetry) {
        telemetry.status = Status::Ok;
    }
}