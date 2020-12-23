use ring::digest::Digest;

pub type PublicKey = [u8; 32];
pub type Hash = Digest;
pub type Seal = Vec<u8>;