#[cfg(feature = "gui")]
pub mod gui;
pub mod frc;
pub mod subsystem;

pub trait RobotBase {
    fn init() -> Self where Self: Sized;
    fn periodic(&mut self);
    fn received_packet(&mut self);
}

enum RobotConnection {
    Simulation,
    Wifi,
}