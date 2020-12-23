use crate::types::{ Hash, Seal, Request, ServerError };
use crate::block::{ MsgBlock };
use std::thread;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Write, Read };

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
                "localhost:8000".to_string(), "localhost:8001".to_string(),
                "localhost:8002".to_string(), "localhost:8003".to_string()
            ]
        }
    }

    pub fn get_request(&mut self, stream: &mut TcpStream) -> Result<Request, ServerError> {
        let mut buff: [u8; 1] = [0; 1];

        match stream.read_exact(&mut buff) {
            Ok(size) => {
                match buff[0] {
                    1 => { Ok(Request::MinerAdd) },
                    2 => { Ok(Request::AdminAdd) },
                    3 => { Ok(Request::Get) },
                    _ => { Err(ServerError::InvalidRequest) }
                }
            },
            Err(_) => {
                Err(ServerError::CannotRead)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn miner_new_test() {
        Miner::new("localhost:8000".to_string());
    }
}