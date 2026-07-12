use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::{
    crypto::encrypt::EncryptedData,
    file::format::{
        LockFile,
        PasswordKeyData,
        WrappedKey,
        MAGIC,
    },
};

const MAX_FIELD_SIZE: u64 = 1024 * 1024 * 1024; // 1 GiB safety limit

// .lock file byte recontruction.
pub fn read_lock_file<P: AsRef<Path>>(
    path: P,
) -> io::Result<LockFile> {

    let mut file = File::open(path)?;

    // Verify file signatures

    // FIRST CHECK: Verify PhaseLock magic bytes
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)?;

    if &magic != MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "not a valid PhaseLock file",
        ));
    }

    // SECOND CHECK: Read format version
    let version = read_u8(&mut file)?;

    if version != 1 {
    return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("unsupported PhaseLock version: {}",version)
    ));
}

    // PROCEED: Read original filename
    let fname_bytes = read_bytes(&mut file)?;

    let original_filename =
        String::from_utf8(fname_bytes)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid filename encoding",
                )
            })?;

    // Read audio-wrapped FileKey
    let audio_wrapped_key = read_wrapped_key(&mut file)?;

    // Read optional password data
    let password_enabled = read_u8(&mut file)?;

    let password_data =
        match password_enabled {
            0 => None,

            1 => {
                let mut salt = [0u8; 16];
                file.read_exact(&mut salt)?;

                let wrapped_key = read_wrapped_key(&mut file)?;

                Some(PasswordKeyData {
                    salt,
                    wrapped_key,
                })
            }

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid password flag",
                ));
            }
        };

    // Read encrypted payload
    let mut payload_nonce = [0u8; 24];
    file.read_exact(&mut payload_nonce)?;

    let payload_ciphertext = read_bytes(&mut file)?;

    let payload = EncryptedData {
        nonce: payload_nonce,
        ciphertext: payload_ciphertext,
    };

    Ok(LockFile {
        version,
        original_filename,
        payload,
        audio_wrapped_key,
        password_data,
    })
}

fn read_wrapped_key<R: Read>(
    reader: &mut R,
) -> io::Result<WrappedKey> {

    let mut nonce = [0u8; 24];
    reader.read_exact(&mut nonce)?;

    let ciphertext = read_bytes(reader)?;

    Ok(WrappedKey {
        nonce,
        ciphertext,
    })
}

fn read_bytes<R: Read>(
    reader: &mut R,
) -> io::Result<Vec<u8>> {

    let mut length_bytes = [0u8; 8];
    reader.read_exact(&mut length_bytes)?;

    let length = u64::from_le_bytes(length_bytes);

    if length > MAX_FIELD_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "field is too large",
        ));
    }

    let length: usize =
        length.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "field size is unsupported",
            )
        })?;

    let mut data = vec![0u8; length];
    reader.read_exact(&mut data)?;

    Ok(data)
}

fn read_u8<R: Read>(
    reader: &mut R,
) -> io::Result<u8> {

    let mut byte = [0u8; 1];

    reader.read_exact(&mut byte)?;

    Ok(byte[0])
}