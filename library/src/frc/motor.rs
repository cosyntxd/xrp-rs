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

    fn received_packet(&mut self) {
        todo!()
    }

    fn sending_packet(&mut self) {
        todo!()
    }
}
