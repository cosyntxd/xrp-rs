// few people are crazy enough to implement a server

use std::net::SocketAddr;
use nt_table::server::BlockingServerHandle;
pub struct NetworkTable {
    table: BlockingServerHandle,
}
impl NetworkTable {
    pub fn bind(ip: SocketAddr) -> Self {
        Self {
            table: BlockingServerHandle::start(ip, todo!()).unwrap()
        }
    }
}