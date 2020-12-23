pub mod types;
pub mod block;
pub mod miner;
use std::net::TcpStream;
use std::io::{Read, Write};
use block_cryptography_rust::signing;
use block::BlockTrait;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let keys = signing::load_key("keys.bin".to_string()).unwrap();

    if args[1] == "miner" {
        let mut m = miner::Miner::new("localhost:8000".to_string());
        m.run();
    } else {
        let mut stream = TcpStream::connect("localhost:8000").unwrap();
        stream.write_all(&[2 as u8][..]).unwrap();

        let string = block::MsgBlock::new("Hello".to_string(), &keys).as_json();
        stream.write_all(string.as_bytes()).unwrap();
    }
}
