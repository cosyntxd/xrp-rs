use std::time::Instant;

use crate::{
    network::{recieve::XRPReceivePacket, send::XRPSendPacket},
    subsystem::{manager::TRACKER, Subsystem, SubsystemTrait},
};
pub struct ConnectionTester {
    pub last_update: Instant
}
impl ConnectionTester {
    pub fn new() -> ConnectionTester {
        ConnectionTester {
            last_update: Instant::now(),
        }
    }
}
impl SubsystemTrait for ConnectionTester {
    fn received_packet(&mut self, packet: &XRPReceivePacket) {
        self.last_update = Instant::now();
    }
}

pub struct Encoder {
    id: u8,
    pub count: i32,
    pub rate: f32,
    pub last_update: Instant,
}
impl Encoder {
    pub fn new(id: u8) -> Encoder {
        Encoder {
            id,
            count: 0,
            rate: 0.0,
            last_update: Instant::now(),
        }
    }
}
impl SubsystemTrait for Encoder {
    fn received_packet(&mut self, packet: &XRPReceivePacket) {
        let encoder = &packet.encoder[self.id as usize];
        if encoder.divisor == 0 || !encoder.has_rate {
            let time = self.last_update.elapsed().as_secs_f32();
            self.rate = (self.count - encoder.count) as f32 / time;
        } else {
            self.rate = encoder.period as f32 / encoder.divisor as f32;
        }
        self.last_update = Instant::now();
        self.count = encoder.count;
    }
}
pub struct Motor {
    id: u8,
    power: f32,
}
impl Motor {
    pub fn new(id: u8) -> Self {
        assert!(id < 4, "Only 4 servos");
        Self { id: id, power: 0.0 }
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

pub struct Servo {
    id: u8,
    power: f32,
}
impl Servo {
    pub fn new(id: u8) -> Self {
        assert!(id < 4, "Only 4 servos");
        Self {
            id: id + 4,
            power: 0.0,
        }
    }
    pub fn set_power(&mut self, voltage: f32) {
        self.power = voltage.clamp(-1.0, 1.0);
    }
}
impl SubsystemTrait for Servo {
    fn sending_packet(&mut self, packet: &mut XRPSendPacket) {
        let dumb_encoding = (self.power + 1.0) / 2.0;
        packet.motor(self.id, dumb_encoding);
    }
}

pub struct EncodedMotor {
    motor: Subsystem<Motor>,
    encoder: Subsystem<Encoder>,
}
impl EncodedMotor {
    pub fn new(id: u8) -> Subsystem<EncodedMotor> {
        let motor = Subsystem::new(Motor::new(id));
        let encoder = Subsystem::new(Encoder::new(id));
        let mut subsystem = Subsystem::new(EncodedMotor {
            motor: motor.clone(),
            encoder: encoder.clone(),
        });
        subsystem.depends_on(&motor.clone());
        subsystem.depends_on(&encoder.clone());
        subsystem
    }
}
impl SubsystemTrait for EncodedMotor {}


