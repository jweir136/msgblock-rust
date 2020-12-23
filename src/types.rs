use ring::digest::Digest;

pub type PublicKey = [u8; 32];
pub type Hash = Digest;
pub type Seal = Vec<u8>;
pub enum Request {
    MinerAdd,
    AdminAdd,
    Get
}
pub enum ServerError {
    InvalidRequest,
    CannotRead
}