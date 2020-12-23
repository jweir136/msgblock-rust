use crate::types::{ Hash, Seal };
use crate::block::{ MsgBlock };
use std::thread;
use std::net::{ TcpListener, TcpStream };

pub struct Miner {
    server: TcpListener,
    addr: String,
    blocks: Vec<MsgBlock>,
    peers: Vec<String>
}

impl Miner {
    pub fn new(ip: String) -> Self {
        Miner {
            server: TcpListener::bind(&ip).expect("Error: Cannot start server"),
            addr: ip,
            blocks: Vec::<MsgBlock>::new(),
            peers: vec![
                "localhost:8000".to_string(), "localhost:8000".to_string(),
                "localhost:8000".to_string(), "localhost:8000".to_string()
            ]
        }
    }
}