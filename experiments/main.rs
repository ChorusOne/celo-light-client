extern crate ethcore_network as net;
extern crate ethcore_network_devp2p as devp2p;
use net::*;
use devp2p::NetworkService;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::thread;
#[macro_use]
extern crate log;

pub const ETH_PROTOCOL_VERSION_64: (u8, u8) = (64, 0x11);
pub const ETH_PROTOCOL: ProtocolId = *b"eth";


struct MyHandler {
    pub got_timeout: AtomicBool,
    pub got_disconnect: AtomicBool,
}

impl NetworkProtocolHandler for MyHandler {
    fn initialize(&self, io: &NetworkContext) {
        io.register_timer(0, Duration::from_secs(1));
    }

    fn read(&self, io: &NetworkContext, peer: &PeerId, packet_id: u8, data: &[u8]) {
        println!("Received {} ({} bytes) from {}", packet_id, data.len(), peer);
    }

    fn connected(&self, io: &NetworkContext, peer: &PeerId) {
        println!("Connected {}", peer);
    }

    fn disconnected(&self, io: &NetworkContext, peer: &PeerId) {
        println!("Disconnected {}", peer);
    }
}

impl MyHandler {
    pub fn new() -> Self {
        MyHandler {
            got_timeout: AtomicBool::new(false),
            got_disconnect: AtomicBool::new(false)
        }
    }
    pub fn got_timeout(&self) -> bool {
        self.got_timeout.load(AtomicOrdering::Relaxed)
    }

    pub fn got_disconnect(&self) -> bool {
        self.got_disconnect.load(AtomicOrdering::Relaxed)
    }
}

fn main () {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let mut cfg = NetworkConfiguration::new_with_port(30304);
    cfg.min_peers = 1;
    cfg.discovery_enabled = false;
    cfg.reserved_nodes = vec![String::from("enode://b115dd2d70d3cf963fa5e5851b9e3edb9be9c9178ef3422d6f12ef74724af35a3d17aab9d8d45c454f65059a72a76c8d5a0db20f229f33e0e70d662e5be0fb2b@127.0.0.1:30303")]; // geth
    //cfg.reserved_nodes = vec![String::from("enode://1caecda2403b589f1798e0b623bc7fcfc16baa7e01c0e1694d8b3878c927362513b9df3de0d79d43838cfcc45932f9838bffcdf36078d11900a6e00dcd731e24@127.0.0.1:30303")]; // celo geth

    let service = NetworkService::new(cfg, None).expect("Error creating network service");
    service.start().expect("Error starting service");

    let handler = Arc::new(MyHandler::new());
    service.register_protocol(handler.clone(), ETH_PROTOCOL, &[ETH_PROTOCOL_VERSION_64]).unwrap();
    println!("{:?}", service.local_url().unwrap());

    while !(handler.got_disconnect()) {
        println!("sleep");
        println!("{:?}", service.connected_peers());
        thread::sleep(Duration::from_millis(500));
    }
}
