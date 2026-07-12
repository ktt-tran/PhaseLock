use crate::crypto::encrypt::EncryptedData;

// File signature to recognize a PhaseLock .lock file.
pub const MAGIC: &[u8; 8] = b"PHASELCK";
pub const FORMAT_VERSION: u8 = 1;

pub struct WrappedKey {
    pub nonce: [u8; 24],
    pub ciphertext: Vec<u8>,
}

pub struct PasswordKeyData {
    pub salt: [u8; 16],
    pub wrapped_key: WrappedKey,
}

pub struct LockFile {
    pub version: u8,

    // File information
    pub original_filename: String,

    // FileKey encrypted with the AudioKey
    pub audio_wrapped_key: WrappedKey,

    // FileKey encrypted with the PasswordKey
    // None when no password was provided
    pub password_data: Option<PasswordKeyData>,

    // File encrypted with the random FileKey
    pub payload: EncryptedData
}

impl From<EncryptedData> for WrappedKey {
    fn from(data: EncryptedData) -> Self {
        Self {
            nonce: data.nonce,
            ciphertext: data.ciphertext,
        }
    }
}