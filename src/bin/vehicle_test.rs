use log::info;
use rover::onboard::vehicle::{Vehicle, debug::DebugVehicle};

fn test(vehicle: &mut impl Vehicle) {
    info!("testing...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("running forwards");
    vehicle.set_motor(1.0);
    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("pausing");
    vehicle.set_motor(0.0);
    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("running backwards");
    vehicle.set_motor(-1.0);
    std::thread::sleep(std::time::Duration::from_secs(2));
    info!("test complete");
}

fn main() {
    rover::initialize_logging();
    test(&mut DebugVehicle {});
}