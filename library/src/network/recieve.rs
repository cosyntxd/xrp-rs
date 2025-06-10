const XRP_TAG_DIO: u8 = 0x14;
const XRP_TAG_ANALOG: u8 = 0x15;
const XRP_TAG_GYRO: u8 = 0x16;
const XRP_TAG_ACCEL: u8 = 0x17;
const XRP_TAG_ENCODER: u8 = 0x18;
// todo: cant properly decode yet
#[derive(Debug, Default)]
pub struct XRPReceivePacket {
    pub sequence: u16,
    pub encoder: [XRPEncoderData; 4],
    pub button_pressed: bool,
    pub gyro: XRPGyroData,
    pub accel: XRPAccelData,
    pub pins: [Option<XRPAnalogData>; 3],
}
impl XRPReceivePacket {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut packet = XRPReceivePacket::default();

        let sequence = u16::from_ne_bytes([bytes[0], bytes[1]]);
        assert!(bytes[2] == 0x0);
        let mut pointer = 3;

        while pointer < bytes.len() {
            let section_size = bytes[pointer] as usize;
            assert!(pointer + section_size <= bytes.len());
            assert!(section_size > 0);

            let tag = bytes[pointer + 1];
            let data = &bytes[pointer + 2..pointer + section_size + 1];
            // println!("{data:#x?}");
            pointer += section_size + 1;

            match tag {
                XRP_TAG_ENCODER => {
                    // todo: yay we have 2 xrps and each decides to send different formats
                    // let decoded = XRPEncoderData::from_bytes(data);
                    // packet.encoder[decoded.device_id as usize] = decoded;
                }
                XRP_TAG_DIO => {
                    packet.button_pressed = data[1] != 0;
                }
                XRP_TAG_GYRO => {
                    packet.gyro = XRPGyroData::from_bytes(data);
                }
                XRP_TAG_ACCEL => {
                    packet.accel = XRPAccelData::from_bytes(data);
                }
                XRP_TAG_ANALOG => {
                    let decoded = XRPAnalogData::from_bytes(data);
                    packet.pins[decoded.device_id as usize] = Some(decoded);
                }
                _ => panic!("Unknown tag:{tag:#04x} (data_len: {section_size}, data:{data:#04x?})"),
            }
        }
        packet
    }
}

fn slice_to_float(bytes: &[u8]) -> f32 {
    assert!(bytes.len() == 4);
    f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// The xrp ships with an older version that doesnt do cool stuff like acceleration.
/// This is supposed to be fixed with the new firmware but i dont think wpilib
/// actually uses the features.
#[derive(Debug, Default, Clone, Copy)]
pub struct XRPEncoderData {
    pub device_id: u8,
    pub count: i32,
    pub has_rate: bool,
    pub period: u32,
    pub divisor: u32,
}
impl XRPEncoderData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            device_id: bytes[0],
            count: i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]),
            has_rate: true,
            period: u32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]),
            divisor: u32::from_be_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct XRPGyroData {
    angles: [f32; 3],
    rates: [f32; 3],
}
impl XRPGyroData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut angles = [0.0; 3];
        let mut rates = [0.0; 3];
        for i in 0..3 {
            angles[i] = slice_to_float(&bytes[8 * i..8 * i + 4]);
            rates[i] = slice_to_float(&bytes[8 * i + 4..8 * i + 8]);
        }
        Self { angles, rates }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct XRPAccelData {
    rates: [f32; 3],
}
impl XRPAccelData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut rates = [0.0; 3];
        for i in 0..3 {
            rates[i] = slice_to_float(&bytes[4 * i..4 * i + 4]);
        }
        Self { rates }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct XRPAnalogData {
    device_id: u8,
    value: f32,
}
impl XRPAnalogData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let device_id = bytes[0];
        assert!(device_id < 3);
        let value = slice_to_float(&bytes[1..5]);
        Self { device_id, value }
    }
}
