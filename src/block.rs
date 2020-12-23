use crate::types::{PublicKey, Hash, Seal};
use block_cryptography_rust::{ hashing, signing };

pub trait BlockTrait {
    fn is_valid(&self, pubkey: &PublicKey) -> bool;
    fn hash(&self) -> Hash;
    fn as_json(&self) -> &str;
    //fn from_json(json: String) -> Self;
}

pub struct MsgBlock {
    msg: String,
    seal: Seal
}

impl BlockTrait for MsgBlock {
    fn is_valid(&self, pubkey: &PublicKey) -> bool {
        signing::verify_data(&pubkey[..], self.hash().as_ref(), &self.seal)
    }

    fn hash(&self) -> Hash {
        hashing::sha256_hash(format!("{}", self.msg).as_bytes())
    }

    fn as_json(&self) -> &str {
        ""
    }
}

impl MsgBlock {
    pub fn new(msg: String, pubkey: &signing::RSAKeyPair) -> Self {
        let seal = signing::sign_data(pubkey, hashing::sha256_hash(&msg.as_bytes()).as_ref());

        MsgBlock {
            msg: msg,
            seal: seal
        }
    }
}