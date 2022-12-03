use rover::onboard::{self, OnboardConfig, vehicle::debug::DebugVehicle};

fn main() {
    rover::initialize_logging();
    let config = OnboardConfig {
        command_endpoint: "tcp://localhost:9311".to_string(),
        telemetry_endpoint: "tcp://localhost:9312".to_string(),
    };

    onboard::run(config, DebugVehicle {});
}

