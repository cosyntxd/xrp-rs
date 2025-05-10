use crate::subsystem::SubsystemTrait;

pub struct Motor {
    id: u8,
}
impl Motor {
    pub fn new() -> Self {
        Self {
            id: 0,
        }
    }
    pub fn move_to(&mut self, x: usize) {
        
    }
}
impl SubsystemTrait for Motor {
    fn periodic(&mut self) {
        // todo!()
    }

    fn received_packet(&mut self) {
        todo!()
    }

    fn sending_packet(&mut self) {
        todo!()
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
