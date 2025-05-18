use std::{net::UdpSocket, time::Instant};

pub mod recieve;
pub mod send;

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
