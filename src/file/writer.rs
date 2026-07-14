use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use crate::file::format::{
    LockFile,
    WrappedKey,
    MAGIC,
};

// Write metadata into .lock file used later to read and recognize encrypted file(s).
pub fn write_lock_file<P: AsRef<Path>>(
    path: P,
    lock_file: &LockFile,
) -> io::Result<()> {

    let mut file = File::create(path)?;

    // Write PhaseLock magic identifier
    file.write_all(MAGIC)?;

    // Write PhaseLock format version
    file.write_all(&[lock_file.version])?;

    // Original filename
    write_bytes(
        &mut file,
        lock_file.original_filename.as_bytes(),
    )?;

    // Audio-wrapped FileKey
    write_wrapped_key(
        &mut file,
        &lock_file.audio_wrapped_key,
    )?;

    // Password section
    match &lock_file.password_data {
        Some(password_data) => {
            // Password enabled
            file.write_all(&[1])?;

            // Argon2 salt
            file.write_all(&password_data.salt)?;

            // Password-wrapped FileKey
            write_wrapped_key(
                &mut file,
                &password_data.wrapped_key,
            )?;
        }

        None => {
            // No password
            file.write_all(&[0])?;
        }
    }

    // Encrypted payload nonce
    file.write_all(&lock_file.payload.nonce)?;

    // Encrypted payload
    write_bytes(
        &mut file,
        &lock_file.payload.ciphertext,
    )?;

    Ok(())
}

fn write_wrapped_key<W: Write>(
    writer: &mut W,
    wrapped_key: &WrappedKey,
) -> io::Result<()> {

    writer.write_all(&wrapped_key.nonce)?;

    write_bytes(
        writer,
        &wrapped_key.ciphertext,
    )
}

fn write_bytes<W: Write>(
    writer: &mut W,
    data: &[u8],
) -> io::Result<()> {

    let length = u64::try_from(data.len())
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "data is too large",
            )
        })?;

    writer.write_all(
        &length.to_le_bytes(),
    )?;

    writer.write_all(data)?;

    Ok(())
}