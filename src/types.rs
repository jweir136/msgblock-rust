use ring::digest::Digest;
use ring::signature::Signature;

pub type PublicKey = [u8; 32];
pub type Hash = Digest;
pub type Seal = Signature;