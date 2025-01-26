
use xrp::*;

pub struct Robot {
    gui: gui::FieldGui,
}
impl RobotBase for Robot {
    fn init() -> Self {
        Self {gui: gui::FieldGui::new()  }
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
}