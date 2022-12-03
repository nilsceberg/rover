use crate::Telemetry;

pub mod debug;

pub trait Vehicle {
    fn set_motor(&mut self, speed: f32);
    fn set_turn(&mut self, turn: f32);
    fn update_telemetry(&self, telemetry: &mut Telemetry);
}