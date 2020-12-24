use crate::types::{ Request, ServerError };
use crate::block::{ MsgBlock, BlockTrait };
use std::thread;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Write, Read };

const KEY: [u8; 32] = [23, 34, 196, 56, 20, 175, 183, 23, 182, 74, 63, 222, 13, 52, 101, 156, 47, 185, 61, 149, 129, 114, 184, 208, 45, 147, 195, 47, 213, 66, 119, 56];

pub struct Miner {
    blocks: Vec<MsgBlock>,
    peers: Vec<String>
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            blocks: Vec::<MsgBlock>::new(),
            peers: vec![
                "localhost:8001".to_string(),
                "localhost:8002".to_string()
            ]
        }
    }

    pub fn get_request(stream: &mut TcpStream) -> Result<Request, ServerError> {
        let mut buff: [u8; 1] = [0; 1];

        match stream.read_exact(&mut buff) {
            Ok(_) => {
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

    pub fn get_block(stream: &mut TcpStream) -> Result<MsgBlock, ServerError> {
        let mut json = String::new();

        match stream.read_to_string(&mut json) {
            Ok(_) => {
                Ok(MsgBlock::from_json(&json[..]))
            },
            Err(_) => {
                Err(ServerError::CannotRead)
            }
        }
    }

    pub fn send_block(&mut self, stream: &mut TcpStream, json: String) -> Result<(), ServerError> {
        match stream.write_all(json.as_bytes()) {
            Ok(_) => { Ok(()) },
            Err(_) => { Err(ServerError::CannotWrite) }
        }
    }

    pub fn add_block(&mut self, stream: &mut TcpStream, block: MsgBlock) {
        for peer in &self.peers {
            println!("\t[INFO] Connecting to {}", &peer);
            match TcpStream::connect(peer) {
                Ok(mut substream) => {
                    substream.write_all(&[1][..]);
                    substream.write_all(&block.as_json().as_bytes());
                },
                Err(_) => {
                    println!("[ERROR] Cannot Connect to Peer");
                }
            };
        }

        if block.is_valid(&KEY[..]) {
            self.blocks.push(block);
        }

        println!("{:?}", &self.blocks);
    }

    fn add_single_block(&mut self, stream: &mut TcpStream, block: MsgBlock) -> Result<(), ServerError> {
        println!("{:?}", &self.blocks);
        if block.is_valid(&KEY[..]) {
            self.blocks.push(block);
            Ok(())
        } else {
            Err(ServerError::InvalidBlock)
        }
    }

    pub fn run(&mut self, ip: String) {
        let server = TcpListener::bind(&ip).unwrap();
        for stream in server.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("[INFO] Client Connected");
    
                    match Miner::get_request(&mut stream) {
                        Ok(request) => {
                            println!("[INFO] Request Recieved");
    
                            match request {
                                Request::AdminAdd => {
                                    match Miner::get_block(&mut stream) {
                                        Ok(block) => {
                                            println!("[INFO] Retrieved Block {:?}", &block);
                                            self.add_block(&mut stream, block);
                                        },
                                        Err(_) => {
                                            println!("[ERROR] Cannot Get Block");
                                        }
                                    };
                                },
                                Request::MinerAdd => {
                                    match Miner::get_block(&mut stream) {
                                        Ok(block) => {
                                            println!("[INFO] Retrieved Block {:?}", &block);
                                            match self.add_single_block(&mut stream, block) {
                                                Ok(()) => {
                                                    println!("[INFO] Added Block");
                                                },
                                                Err(_) => {
                                                    println!("[ERROR] Cannot Add Block");
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            println!("[ERROR] Cannot get Block");
                                        }
                                    };
                                },
                                Request::Get => {
                                    println!("[INFO] Requested Block");
                                    let mut buff = [0 as u8; 1];
                                    match stream.read_exact(&mut buff) {
                                        Ok(_) => {
                                            println!("[INFO] Requested Element {}", &buff[0]);
                                            match &self.blocks.get(buff[0] as usize) {
                                                Some(block) => {
                                                    match self.send_block(&mut stream, block.as_json()) {
                                                        Ok(_) => {
                                                            println!("[INFO] Block Sent");
                                                        },
                                                        Err(_) => {
                                                            println!("[ERROR] Block could not be sent");
                                                        }
                                                    };       
                                                },
                                                None => {
                                                    println!("[ERROR] Block not found. Perhaps it must propigate to the network");
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            println!("[ERROR] Cannot get ID");      
                                        }
                                    };
                                },
                                _ => {
    
                                }
                            };
                        },
                        Err(_) => {
                            println!("[ERROR] Invalid Request");
                        }
                    };
                },
                Err(_) => {
                    println!("[ERROR] Client Cannot Connect");
                }
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