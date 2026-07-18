use std::{
    io,
    io::{Cursor},
    path::Path,
    sync::mpsc::Sender,
};

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
    XNonce,
};

use crate::{
    audio::key::derive_audio_key,
    crypto::{
        archive::extract_archive,
        key::{
            FileKey,
            KEY_SIZE,
        },
        encrypt::EncryptedData,
        password::derive_password_key,
    },
    file::{
        format::WrappedKey,
        reader::read_lock_file,
    },
};

pub fn decrypt_bytes(
    encrypted: &EncryptedData,
    key: &[u8; 32],
) -> Result<Vec<u8>, chacha20poly1305::Error> {

    // Retrieve key representation from hash key and retrieve the nonce.
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XNonce::try_from(&encrypted.nonce[..]).expect("24 byte nonce");

    cipher.decrypt(
        &nonce,
        encrypted.ciphertext.as_ref(),
    )
}

pub fn unwrap_file_key(
    wrapped_key: &EncryptedData,
    wrapping_key: &[u8; 32],
) -> Result<FileKey, chacha20poly1305::Error> {

    let decrypted =
        decrypt_bytes(
            wrapped_key,
            wrapping_key,
        )?;

    let key_bytes: [u8; KEY_SIZE] =
        decrypted
            .try_into()
            .map_err(|_| chacha20poly1305::Error)?;

    Ok(FileKey::from_bytes(key_bytes))
}

// Function converts WrappedKey type into EncryptedData type
// understood by encrypt_bytes and decrypte_bytes.
fn wrapped_key_to_encrypted_data(
    wrapped_key: &WrappedKey,
) -> EncryptedData {
    EncryptedData {
        nonce: wrapped_key.nonce,
        ciphertext: wrapped_key.ciphertext.clone(),
    }
}

/// Complete extraction with audio file.
pub fn decrypt_with_audio<L: AsRef<Path>, A: AsRef<Path>>(
    lock_path: L,
    audio_path: A,
    sender: &Sender<String>,
) -> Result<Vec<u8>, io::Error> {

    // Read and parse the .lock file.
    sender.send("Reading lock...".to_string()).ok();
    let lock_file = read_lock_file(lock_path)?;

    // Derive the key from the provided audio file.
    let audio_key = derive_audio_key(audio_path)?;

    // Convert the stored wrapped key.
    let wrapped_file_key =
        wrapped_key_to_encrypted_data(
            &lock_file.audio_wrapped_key,
        );

    // Recover the original random FileKey.
    let file_key =
        unwrap_file_key(
            &wrapped_file_key,
            audio_key.as_bytes(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "incorrect audio key",
            )
        })?;

    sender.send("Reading .lock...".to_string()).ok();

    // Decrypt the actual file contents
    sender.send("Extracting...".to_string()).ok();
    let decrypted =
        decrypt_bytes(
            &lock_file.payload,
            file_key.as_bytes(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "failed to decrypt archive",
            )
        })?;

    // Decompress archive.
    let decompressed_archive = zstd::decode_all(Cursor::new(decrypted));
    
    decompressed_archive
}

/// Complete extraction with password.
pub fn decrypt_with_password<L: AsRef<Path>>(
    lock_path: L,
    password: &str,
    sender: &Sender<String>,
) -> Result<Vec<u8>, io::Error> {

    // Read the .lock file
    sender.send("Reading .lock...".to_string()).ok();
    let lock_file = read_lock_file(lock_path)?;

    // Check if this .lock file has password recovery enabled.
    let password_data = match lock_file.password_data.as_ref() {
        Some(pw) => pw,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "this file was not encrypted with a password option",
            ));
        }
    };

    // Recreate the PasswordKey using the stored salt.
    let password_key =
        derive_password_key(
            password,
            &password_data.salt,
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "failed to derive password key",
            )
        })?;

    // Convert the stored wrapped FileKey.
    let wrapped_file_key = wrapped_key_to_encrypted_data(&password_data.wrapped_key);

    // Try to recover the original FileKey.
    let file_key =
        unwrap_file_key(
            &wrapped_file_key,
            password_key.as_bytes(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "incorrect password",
            )
        })?;

    // Decrypt the actual file.
    sender.send("Extracting...".to_string()).ok();
    let decrypted =
        decrypt_bytes(
            &lock_file.payload,
            file_key.as_bytes(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "failed to decrypt archive",
            )
        })?;

    // Decompress archive.
    let decompressed_archive = zstd::decode_all(Cursor::new(decrypted));

    decompressed_archive

}

// Extract all data and restore into the output directory.
pub fn decrypt_and_extract_with_audio<L: AsRef<Path>, A: AsRef<Path>, O: AsRef<Path>>(
    lock_path: L,
    audio_path: A,
    output_directory: O,
    sender: &Sender<String>,
) -> io::Result<()> {

    let decrypted_archive = decrypt_with_audio(
        lock_path,
        audio_path,
        sender,
    )?;

    extract_archive(
        &decrypted_archive,
        output_directory.as_ref(),
    )?;

    Ok(())
}

// Extract all data and restore into the output directory.
pub fn decrypt_and_extract_with_password<L: AsRef<Path>, O: AsRef<Path>>(
    lock_path: L,
    password: &str,
    output_directory: O,
    sender: &Sender<String>,
) -> io::Result<()> {

    let decrypted_archive = decrypt_with_password(
        lock_path,
        password,
        sender,
    )?;

    extract_archive(
        &decrypted_archive,
        output_directory.as_ref(),
    )?;

    Ok(())
}