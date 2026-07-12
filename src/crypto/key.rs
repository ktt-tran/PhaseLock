use rand::RngCore;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const KEY_SIZE: usize = 32;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct FileKey {
    bytes: [u8; KEY_SIZE],
}

impl FileKey {
    pub fn generate() -> Self {
        let mut bytes = [0u8; KEY_SIZE];

        rand::rng().fill_bytes(&mut bytes);

        Self { bytes }
    }

    pub fn from_bytes(bytes: [u8; KEY_SIZE]) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.bytes
    }
}
