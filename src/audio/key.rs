// Hash key generation module.

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
    bytes: [u8; AUDIO_KEY_SIZE],
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

    // Loops through the file and reading the entire file by chunck-by-chunck to the end.
    // Each loop reads 8192 bits chunck and updates the hash state which will be the hash
    // key generated.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn same_file_produces_same_key() {
        let path = "test_audio_key.bin";

        fs::write(path, b"fake audio data").unwrap();

        let key1 = derive_audio_key(path).unwrap();
        let key2 = derive_audio_key(path).unwrap();

        assert_eq!(
            key1.as_bytes(),
            key2.as_bytes()
        );

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn different_files_produce_different_keys() {
        let path1 = "test_audio_1.bin";
        let path2 = "test_audio_2.bin";

        fs::write(path1, b"audio file one").unwrap();
        fs::write(path2, b"audio file two").unwrap();

        let key1 = derive_audio_key(path1).unwrap();
        let key2 = derive_audio_key(path2).unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes()
        );

        fs::remove_file(path1).unwrap();
        fs::remove_file(path2).unwrap();
    }
}