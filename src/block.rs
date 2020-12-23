use crate::types::{PublicKey, Hash, Seal};
use block_cryptography_rust::{ hashing, signing };

pub trait BlockTrait {
    fn is_valid(&self, pubkey: &PublicKey) -> bool;
    fn hash(&self) -> Hash;
    fn as_json(&self) -> &str;
    //fn from_json(json: String) -> Self;
}

pub struct MsgBlock {
    id: usize,
    msg: String,
    seal: Seal
}

impl BlockTrait for MsgBlock {
    fn is_valid(&self, pubkey: &PublicKey) -> bool {
        signing::verify_data(&pubkey[..], self.hash().as_ref(), &self.seal)
    }

    fn hash(&self) -> Hash {
        hashing::sha256_hash(format!("{}{}", self.id, self.msg).as_bytes())
    }

    fn as_json(&self) -> &str {
        ""
    }
}