use xrp::{frc::motor::Motor, *};

pub struct Robot {
    left_motor: u8,
    right_motor: u8,
    gui: gui::FieldGui,
}
impl RobotBase for Robot {
    fn init() -> Self {
        Self {
            left_motor: 0,
            right_motor: 0,
            gui: gui::FieldGui::new(),
        }
    }

    fn periodic(&mut self) {
        todo!()
    }

    fn received_packet(&mut self) {
        todo!()
    }
}

fn main() {
    let robot = Robot::init();
    let a = subsystem::Subsystem::new(Motor::new());
}
