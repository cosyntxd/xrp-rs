// #[cfg(feature = "gui")]
// pub mod gui;
#[cfg(feature = "nt")]
pub mod nt;

pub mod frc;
pub mod network;
pub mod subsystem;
// pub mod test;
pub trait RobotBase {
    fn init() -> Self
    where
        Self: Sized;
    fn periodic(&mut self);
    fn received_packet(&mut self);
}
