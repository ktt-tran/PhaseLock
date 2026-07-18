use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

// zeroize carete used to securely erase/zero memory.
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const AUDIO_KEY_SIZE: usize = 32;

// zeroize key when out of scope.
#[derive(Zeroize, ZeroizeOnDrop)]
// Key struct that 32 bytes in length.
pub struct AudioKey {
    bytes: [u8; AUDIO_KEY_SIZE]
}

impl AudioKey {
    pub fn as_bytes(&self) -> &[u8; AUDIO_KEY_SIZE] {
        &self.bytes
    }
}

pub fn derive_audio_key<P: AsRef<Path>>(
    path: P,
) -> Result<AudioKey, io::Error> {

    let mut file = File::open(path)?;

    // Creates hasher using the file and context string for PhaseLock Audio Key.
    let mut hasher = blake3::Hasher::new_derive_key(
        "PhaseLock Audio Key v1"
    );

    let mut buffer = [0u8; 8192];

    // Hash state updates creates hash key.
    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();

    // Save the final hash key.
    let mut key = [0u8; AUDIO_KEY_SIZE];
    key.copy_from_slice(hash.as_bytes());

    Ok(AudioKey { bytes: key })
}