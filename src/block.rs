use crate::types::{Hash, Seal};
use block_cryptography_rust::{ hashing, signing };
use std::fmt::{ Debug, Result, Formatter };
#[allow(unused_imports)]
use ring::signature::KeyPair;
use serde::{ Serialize, Deserialize };
use serde_json::{ to_string, from_str };

pub trait BlockTrait {
    fn is_valid(&self, pubkey: &[u8]) -> bool;
    fn hash(&self) -> Hash;
    fn as_json(&self) -> String;
    fn from_json(json: &str) -> Self;
}

#[derive(Serialize, Deserialize)]
pub struct MsgBlock {
    msg: String,
    pub seal: Seal
}

impl BlockTrait for MsgBlock {
    fn is_valid(&self, pubkey: &[u8]) -> bool {
        signing::verify_data(pubkey.as_ref(), self.hash().as_ref(), &self.seal.as_ref())
    }

    fn hash(&self) -> Hash {
        hashing::sha256_hash(format!("{}", &self.msg).as_bytes())
    }

    fn as_json(&self) -> String {
        to_string(&self).unwrap()
    }

    fn from_json(json: &str) -> Self {
        let block: MsgBlock = from_str(json).unwrap();
        block
    }
}

impl MsgBlock {
    pub fn new(msg: String, pubkey: &signing::RSAKeyPair) -> Self {
        let seal = signing::sign_data(pubkey, hashing::sha256_hash(&msg.as_bytes()).as_ref());
        let mut sealbuff = Vec::<u8>::new();
        
        for i in 0..seal.as_ref().len() {
            sealbuff.push(seal.as_ref()[i]);
        }

        MsgBlock {
            msg: msg,
            seal: sealbuff
        }
    }
}

impl Debug for MsgBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("MsgBlock").field("msg", &self.msg).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msgblock_new_test() {
        let keypair = signing::load_key("keys.bin".to_string()).unwrap();
        let block = MsgBlock::new("Hello World!".to_string(), &keypair);
        assert_eq!(format!("{:?}", &block), "MsgBlock { msg: \"Hello World!\" }");
    }

    #[test]
    fn msgblock_checking_test() {
        let keypair = signing::load_key("keys.bin".to_string()).unwrap();
        let block = MsgBlock::new("Hello World!".to_string(), &keypair);

        assert_eq!(block.is_valid(keypair.public_key().as_ref()), true);
    }

    #[test]
    fn msgblock_json_test() {
        let keypair = signing::load_key("keys.bin".to_string()).unwrap();
        let block = MsgBlock::new("Hello World!".to_string(), &keypair);

        let json = &block.as_json();
        let newblock = MsgBlock::from_json(&json[..]);

        assert_eq!(format!("{:?}", block), format!("{:?}", newblock));
    }
}