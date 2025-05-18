// few people are crazy enough to implement a working server
use frclib_core::value::IntoFrcValue;
use std::{net::SocketAddr, ops::{Deref, DerefMut}};
use nt_table::{client, server::BlockingServerHandle, server::config::ServerConfig};
use std::net::{ToSocketAddrs};

pub struct NetworkTable {
    table: BlockingServerHandle,
}

impl NetworkTable {
    pub fn bind(addr: &str) -> Self {
        let server_addr = addr
            .to_socket_addrs()
            .unwrap()
            .next()
            .expect("Invalid address");
        Self {
            table: BlockingServerHandle::start(server_addr, ServerConfig::default()).unwrap(),
        }
    }
}

impl Deref for NetworkTable {
    type Target = BlockingServerHandle;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for NetworkTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}