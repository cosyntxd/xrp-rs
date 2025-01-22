pub trait RobotBase {
    fn init() -> Self where Self: Sized;
    fn periodic(&mut self);
    fn received_packet(&mut self);
}
