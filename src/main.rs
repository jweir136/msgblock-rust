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
        let mut m = miner::Miner::new();
        m.run(args[2].to_string());
    } else {
        let mut stream = TcpStream::connect("localhost:8000").unwrap();
        stream.write_all(&[2 as u8][..]).unwrap();

        let string = block::MsgBlock::new("Hello".to_string(), &keys).as_json();
        stream.write_all(string.as_bytes()).unwrap();

        drop(stream);

        let mut stream = TcpStream::connect("localhost:8000").unwrap();
        stream.write_all(&[3 as u8][..]).unwrap();
        stream.write_all(&[0 as u8][..]).unwrap();

        let mut buff = String::new();
        stream.read_to_string(&mut buff).unwrap();
        println!("{}", buff);
    }
}
