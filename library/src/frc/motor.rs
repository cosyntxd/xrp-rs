use crate::{network::{recieve::XRPReceivePacket, send::XRPSendPacket}, subsystem::SubsystemTrait};

pub struct Motor {
    id: u8,
}
impl Motor {
    pub fn new(id: usize) -> Self {
        Self {
            id: id as u8,
        }
    }
    pub fn move_to(&mut self, x: usize) {
        
    }
}
impl SubsystemTrait for Motor {
    fn periodic(&mut self) {}

    fn received_packet(&mut self, packet: &XRPReceivePacket) {
        // packet.encoder
    }

    fn sending_packet(&mut self, packet: &mut XRPSendPacket) {
        packet.motor(self.id, 0.0);
    }
}
