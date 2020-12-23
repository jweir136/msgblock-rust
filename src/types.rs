use ring::digest::Digest;

pub type PublicKey = [u8; 32];
pub type Hash = Digest;
pub type Seal = [u8; 64];