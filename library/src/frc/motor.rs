use crate::subsystem::SubsystemTrait;

pub struct Motor {
    id: u8,
}
impl Motor {
    pub fn new() -> Self {
        todo!()
    }
}
impl SubsystemTrait for Motor {
    fn periodic(&mut self) {
        todo!()
    }

    fn on_packet(&mut self) {
        todo!()
    }
}