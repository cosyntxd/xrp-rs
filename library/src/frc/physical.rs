use crate::{network::{recieve::XRPReceivePacket, send::XRPSendPacket}, subsystem::SubsystemTrait};

//
pub struct Encoder {   
    id: u8,
}
impl Encoder {
    pub fn new() {
        
    }
}
impl SubsystemTrait for Encoder {
    fn received_packet(&mut self, packet: &XRPReceivePacket) {
        // packet.encoder[self.id as usize];
        // umm
    }
}

//
pub struct Motor {
    id: u8,
    power: f32,
}
impl Motor {
    pub fn new(id: u8) -> Self {
        Self {
            id: id,
            power: 0.0,
        }
    }
    pub fn set_power(&mut self, voltage: f32) {
        self.power = voltage.clamp(-1.0, 1.0);
    }
}
impl SubsystemTrait for Motor {
    fn sending_packet(&mut self, packet: &mut XRPSendPacket) {
        packet.motor(self.id, self.power);
    }
}

// 
pub struct Servo {
    
}