use std::{net::UdpSocket, time::Instant};

pub mod recieve;
pub mod send;
// Technically the xrp is able to run properly run when the sent packets do not contain
// all the tags. The poor documentation also seems to support this, however it *might*
// cause some UB. Im not sure cuz i didnt read through the firmware's repo. It seems
// like its safe, but keep this in mind if something weird happens.

// All network facing code is in rust, therefor it is 100% safe and no CVEs

enum RobotConnection {
    Simulation,
    Wifi,
    AccessPoint, // todo: i have no idea how this works
}

pub struct XRPConnection {
    // inner: RobotConnection,
    socket: UdpSocket,
    last_sent: Instant,
    last_recieved: Instant,
}
