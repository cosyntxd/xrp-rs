use std::{
    net::UdpSocket,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::network::{recieve::XRPReceivePacket, send::XRPSendPacket};

pub mod recieve;
pub mod send;

pub struct XRPConnection {
    packet_receiver: Receiver<XRPReceivePacket>,
    socket: UdpSocket,
    handle: JoinHandle<()>,
    last_recieved: Arc<Mutex<Instant>>,
    last_sent: Instant,
    address: String,
}
impl XRPConnection {
    pub fn new(address: &'static str) -> Option<Self> {
        let socket = UdpSocket::bind(address).ok()?;
        let socket_clone = socket.try_clone().unwrap();

        let (tx, rx) = mpsc::channel();

        let last_recieved = Arc::new(Mutex::new(Instant::now()));
        let last_recieved_clone = Arc::clone(&last_recieved);
        let handle = thread::spawn(move || {
            let mut buf = [0; 512];
            while let Ok((len, addr)) = socket_clone.recv_from(&mut buf) {
                // println!("{len}");
                // println!("{:?}", buf);
                // if let Err(err) = tx.send(XRPReceivePacket::from_bytes(&mut buf)) {
                //     eprintln!("Could not send packet through channel: {err:?}");
                // } else {
                *last_recieved_clone.lock().unwrap() = Instant::now();
                // }
            }
        });

        let connection = Self {
            packet_receiver: rx,
            socket,
            handle,
            last_recieved,
            last_sent: Instant::now(),
            address: String::new()
        };
        Some(connection)
    }
    pub fn send(&mut self, data: XRPSendPacket) {
        self.socket
            .send_to(&data.build_packet(), self.address.clone())
            .unwrap();
        self.last_sent = Instant::now();
    }
    pub fn last_recieved(&self) -> Duration {
        Instant::now().duration_since(*self.last_recieved.lock().unwrap())
    }
    pub fn last_sent(&self) -> Duration {
        Instant::now().duration_since(self.last_sent)
    }
    pub fn receive(&self) -> XRPReceivePacket {
        self.packet_receiver.recv().unwrap()
    }
    pub fn try_receive(&self) -> Option<XRPReceivePacket> {
        self.packet_receiver.try_recv().ok()
    }
}
