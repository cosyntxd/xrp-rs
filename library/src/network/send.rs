const XRP_TAG_MOTOR: u8 = 0x12;
const XRP_TAG_SERVO: u8 = 0x13;
const XRP_TAG_DIO: u8 = 0x14;

#[derive(Debug)]
pub struct XRPSendPacket {
    data: Vec<u8>,
}
impl XRPSendPacket {
    pub fn new(sequence: u16, control: bool) -> Self {
        Self {
            data: vec![
                (sequence >> 8) as u8,
                (sequence & 0xff) as u8,
                control as u8,
            ],
        }
    }
    pub fn add_tag(&mut self, tag: XRPTaggedDataEnum) {
        tag.write_to(&mut self.data);
    }
    pub fn motor(&mut self, channel: u8, value: f32) {
        self.add_tag(XRPTaggedDataEnum::Motor { channel, value });
    }
    pub fn servo(&mut self, channel: u8, value: f32) {
        self.add_tag(XRPTaggedDataEnum::Servo { channel, value });
    }
    pub fn dio(&mut self, channel: DioPin, value: bool) {
        self.add_tag(XRPTaggedDataEnum::Dio {
            channel: channel as u8,
            value,
        });
    }
    pub fn build_packet(self) -> Vec<u8> {
        self.data
    }
}
pub enum DioPin {
    UserButton = 0x0,
    OnboardLed = 0x1,
    Reserved1 = 0x2,
    Reserved2 = 0x3,
    LeftEncoderA = 0x4,
    LeftEncoderB = 0x5,
    RightEncoderA = 0x6,
    RightEncoderB = 0x7,
    Motor3EncoderA = 0x8,
    Motor3EncoderB = 0x9,
    Motor4EncoderA = 0xa,
    Motor4EncoderB = 0xb,
}
pub enum XRPTaggedDataEnum {
    Motor { channel: u8, value: f32 },
    Servo { channel: u8, value: f32 },
    Dio { channel: u8, value: bool },
}
impl XRPTaggedDataEnum {
    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.payload_size());
        buffer.push(self.tag());
        match self {
            XRPTaggedDataEnum::Motor { channel, value } => {
                buffer.push(*channel);
                buffer.extend_from_slice(&value.to_be_bytes());
            }
            XRPTaggedDataEnum::Servo { channel, value } => {
                buffer.push(*channel);
                buffer.extend_from_slice(&value.to_be_bytes());
            }
            XRPTaggedDataEnum::Dio { channel, value } => {
                buffer.push(*channel);
                buffer.push(*value as u8);
            }
        }
    }
    pub fn tag(&self) -> u8 {
        match self {
            XRPTaggedDataEnum::Motor { .. } => XRP_TAG_MOTOR,
            XRPTaggedDataEnum::Servo { .. } => XRP_TAG_SERVO,
            XRPTaggedDataEnum::Dio { .. } => XRP_TAG_DIO,
        }
    }
    pub fn payload_size(&self) -> u8 {
        self.data_size() + 1
    }
    pub fn data_size(&self) -> u8 {
        match self {
            XRPTaggedDataEnum::Motor { .. } => 5,
            XRPTaggedDataEnum::Servo { .. } => 5,
            XRPTaggedDataEnum::Dio { .. } => 2,
        }
    }
}
