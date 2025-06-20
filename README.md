# XRP-rs
This repositry is an attempt to rewrite a small subset of the libraries from [wpilib](https://github.com/wpilibsuite/allwpilib) in rust - enough to get the [xrp](https://www.sparkfun.com/experiential-robotics-platform-xrp-kit-beta.html) robot to drive around from wasd input or follow a line. By no means is this a replacement for the full WPILib ecosystem, its just a fun project to learn rust and see how far I can go. Official documentation or programming the xrp in java can be found [here](https://docs.wpilib.org/en/latest/docs/xrp-robot/index.html), the rust implementation is meant to be similar but not exactly the same.

# Updating firmware
When you recieve your xrp, it should be running a recent enough version that will work. However, you may want to upgrade to upgrade to the newest version to get some additional features or bug fixes. To install the latest firmware, do the following: 
- Git clone https://github.com/wpilibsuite/xrp-wpilib-firmware
- Connect the XRP to your computer via USB
- Hold the BOOTSEL and press the RESET button
- It should appear as USB storage device
- Open Visual Studio Code and install platformIO
- Press ^⌥U to upload the firmware

# Example
Contructs a simple tank drive that is controlled by the left joystick and powers the left and right motors. It also calculates the odometry and logs it to the network table. See a more complete example in robot/
```rs
pub struct TankDrive {
    left: Subsystem<EncodedMotor>,
    right: Subsystem<EncodedMotor>,
    gyro: Subsystem<Gyro>,
    joystick: Joystick,
    #[nt_log]
    pose: Pose2d,
    power_scale: f32,

}
impl TankDrive {
    pub fn new() -> TankDrive {
        TankDrive {
            left: Subsystem::new(EncodedMotor::new(0)),
            right: Subsystem::new(EncodedMotor::new(1)),
            gyro: Subsystem::new(Gyro::new()),
            joystick: Joystick::new(0),
            pose: Pose2d::new(),
            power_scale: 1.0,
        }
    }
    pub fn set_power(&mut self, power: f32) {
        self.power_scale = power;
    }
}
impl SubsystemTrait for TankDrive {
    fn periodic(&mut self, dt: f32) {
        let left_power = power_scale * (self.joystick.y + self.joystick.x) as f32;
        let right_power = power_scale * (self.joystick.y - self.joystick.x) as f32;

        self.left.write().set_power(left_power);
        self.right.write().set_power(right_power);

        let left_count = self.left.read().get_rate() * dt;
        let right_count = self.right.read().get_rate() * dt;

        let delta_distance = (left_distance + right_distance) / 2.0;
        let delta_theta = self.gyro.read().get_heading();

        self.pose = self.pose.exp(delta_distance, delta_theta);
    }
}
```
# Usage
Why wouldnt you install this fun rust library?
- download and install rust
- download and install rust the language
- git clone this repo
- python3 nt_setup.py
- cd robot
- cargo r

# Protocol
The XRP by default will bind to localhost:3540 and both the library and the robot communicate over udp. It uses a binary-based protocol due to performance limitations. Also since there is no handshaking, they basically scream at each other and hope the other side is listening. This makes the protocol really easy to implement (~200 lines for basic implementation) and is also pretty fun to write. Specifcation is laid out below.

## Packet format
| Field Name  | Size	| Notes  |
|-------------|---------|--------|
| Sequence    | u16     | Packets with sequence numbers ≤ current maximum are discarded unless rollover is detected |
| Control     | bool    | Robot enable/disable state (1 = enabled, 0 = disabled) |
| Tagged Data | 0-8190  | Packets will be truncated to 8192 + 1 byes and 3 are already used|

If a packet is cut off, and its payload is not included, then it will read past the buffer and cause undefined behavior. 


## Tagged Data
XRP parses tags in order, so if there are two commands changing the same motor's speed, the first will be overwritten. Although every tag has a channel, it makes more sense to document it per tag even though it will likely remain the case in the future.
| Field Name | Size | Notes                                |
|------------|------|--------------------------------------|
| Size       | u8   | This byte is excluded in calculation  |
| TagID      | u8   | Corresponds to what paylod describes |
| Payload    | 0-254| Each tagged has its own payload size |


| Tag  | Description | Direction |
|------|-------------|-----------|
| 0x12 | Motor       | Outbound  |
| 0x13 | Servo       | Outbound  |
| 0x14 | DIO         | Outbound  |
| 0x15 | Analog      | Inbound   |
| 0x16 | Gyro        | Inbound   |
| 0x17 | Accel       | Inbound   |
| 0x18 | Encoder     | Inbound   |


### Motor
| Name    | Size | Notes                  | 
|---------|------|------------------------|
| Channel | u8   | See table for mappings |
| Power   | f32  | -1.0 to 1.0            |

| ID | Description |
|----|-------------|
| 0  | Left Motor  |
| 1  | Right Motor |
| 2  | Motor 3*    |
| 3  | Motor 4*    |

*The motor numbers might not be labeled correctly on pcb, idk

### Servo
| Name    | Size | Notes        | 
|---------|------|--------------|
| Channel | u8   | 0-3 are for motors, 4-7 are servos |
| Power   | f32  | 0.0 to 1.0 but internally maps to -1.0 to 1.0 |

### DIO
| Name    | Size | Notes                 | 
|---------|------|-----------------------|
| Channel | u8   | See table for mapping |
| Value   | bool | enable/disable state (1 = enabled, 0 = disabled) |

| Value | Description    |
|-------|----------------|
| 0x0   | UserButton     |
| 0x1   | OnboardLed     |
| 0x2   | Reserved1      |
| 0x3   | Reserved2      |
| 0x4   | LeftEncoderA   |
| 0x5   | LeftEncoderB   |
| 0x6   | RightEncoderA  |
| 0x7   | RightEncoderB  |
| 0x8   | Motor3EncoderA |
| 0x9   | Motor3EncoderB |
| 0xa   | Motor4EncoderA |
| 0xb   | Motor4EncoderB |

Only the OnboardLed actually works, the rest are ignored. Nothing is said about this in the documentation

### Analog
| Name    | Size | Notes                         |
|---------|------|-------------------------------|
| Channel | u8   | See table for mapping         |
| Value   | f32  | Reads voltage; -1.0 is uninit unless optional |

| Channel | Name | Max voltage | Optional |
|------|---------|------------|-----------|
| 0x0  | ReflectanceLeft | 5.0   | No |
| 0x1  | ReflectanceRight | 5.0   | No |
| 0x2  | Rangefinder | 5.0*  | Yes |

*Not entirely certain if the 5.0 limit is enforced anywhere

### Gyro
| Name | Size    | Notes       |
|------|---------|-------------|
| XYZ  | f32 x 3 | Rate (dps)  |
| XYZ  | f32 x 3 | Angle (deg) |

### Accel
| Name | Size    | Notes       |
|------|---------|-------------|
| XYZ  | f32 x 3 | Accel (g)   |

### Encoder
There is actually two different formats that can be sent depending on what version of firmware is currently running. If you compiled from source you are running the newer format, with more info. If you havent updated, or updated to prebuilt binaries you are using the older format. Note: Motor 0's direction is always flipped in the firmware
| Name  | Size | Notes |
|-------|------|-------|
| ID    | u8   | See motor for ID naming |
| Count | u32  | 12 CPR |

| Name               | Size | Notes                   |
|--------------------|------|-------------------------|
| ID                 | u8   | See motor for ID naming |
| Count              | i32  | 12 CPR                  |
| Period Numerator   | u32  | Use math for velocity   |
| Period Denominator | u32  | Use math for velocity   |
