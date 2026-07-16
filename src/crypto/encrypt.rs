use std::{
    io,
    path::{Path, PathBuf},
};

use chacha20poly1305::{
    aead::{Aead, Generate, KeyInit},
    XChaCha20Poly1305,
    XNonce,
};

use crate::{
    audio::key::derive_audio_key,
    crypto::{
        archive,
        key::FileKey,
        password::{
            derive_password_key,
            generate_salt,
        },
    },
    file::{
        format::{
            LockFile,
            PasswordKeyData,
            WrappedKey,
            FORMAT_VERSION,
        },
        writer::write_lock_file,
    },
};

pub const NONCE_SIZE: usize = 24;

pub struct EncryptedData {
    pub nonce: [u8; NONCE_SIZE],
    pub ciphertext: Vec<u8>,
}

// General purpose encryption.
pub fn encrypt_bytes(
    plaintext: &[u8],
    key: &[u8; 32],
) -> Result<EncryptedData, chacha20poly1305::Error> {

    // Converts hash key into key representation expected by the cryptography library.
    let cipher = XChaCha20Poly1305::new(key.into());

    let nonce = XNonce::generate();

    // Encrypt plaintext message with a 24 bit generated nonce.
    let ciphertext = cipher.encrypt(
        &nonce,
        plaintext,
    )?;

    Ok(EncryptedData {
        nonce: nonce.into(),
        ciphertext,
    })
}

// File key wrapping protects plain key being seen in metadata.
// The origin data will be encrypted with an audio file (and option
// password) is used as the wrapping key.
pub fn wrap_file_key(
    file_key: &FileKey,
    wrapping_key: &[u8; 32],
) -> Result<EncryptedData, chacha20poly1305::Error> {

    encrypt_bytes(
        file_key.as_bytes(),
        wrapping_key,
    )
}

pub fn encrypt_with_key<A: AsRef<Path>, O: AsRef<Path>>(
    input_data: &[PathBuf],
    audio_path: A,
    output_path: O,
    password: Option<&str>,
) -> io::Result<()> {

    // Generate random key for file.
    let file_key = FileKey::generate();

    // Create archive for selected files/folders.
    println!("Creating archive...");
    let archive_data = archive::create_archive(input_data)?;
    println!("Archive created.");

    // Encrypt the archive of files/folders with the file_key byte code.
    let payload = encrypt_bytes(
        &archive_data,
        file_key.as_bytes(),
    )
    // Convert chacha20poly1305::Error into io:Error if file
    // ecryption is unsuccessful.
    .map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            "failed to encrypt file",
        )
    })?;

    // Derive key from exact audio file.
    println!("Deriving audio key...");
    let audio_key = derive_audio_key(audio_path)?;
    println!("Audio key derived.");

    // Encrypt/wrap the FileKey using AudioKey.
    let audio_wrapped_key = WrappedKey::from(
        wrap_file_key(
            &file_key,
            audio_key.as_bytes(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "failed to wrap file key with audio key",
            )
        })?
    );

    // Optionally wrap the same FileKey using.
    let password_data =
        if let Some(password) = password {

            // Add salt to password hash.
            let salt = generate_salt();

            let password_key =
                derive_password_key(
                    password,
                    &salt,
                )
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "failed to derive password key",
                    )
                })?;

            let wrapped_key = WrappedKey::from(
                wrap_file_key(
                    &file_key,
                    password_key.as_bytes(),
                )
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "failed to wrap file key with password key",
                    )
                })?
            );

            Some(PasswordKeyData {
                salt,
                wrapped_key,
            })

        } else {
            None
        };

    // Build the complete PhaseLock structure.
    let lock_file = LockFile {
        version: FORMAT_VERSION,
        audio_wrapped_key,
        password_data,
        payload
    };

    // Write the final .lock file.
    println!("Writing lock file...");
    write_lock_file(output_path, &lock_file)?;
    println!("Lock file written.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn encryption_changes_the_data() {
        let key = [42u8; 32];
        let plaintext = b"PhaseLock secret data";

        let encrypted =
            encrypt_bytes(plaintext, &key).unwrap();

        assert_ne!(
            encrypted.ciphertext,
            plaintext
        );
    }

    #[test]
    fn encryption_uses_different_nonces() {
        let key = [42u8; 32];
        let plaintext = b"same data";

        let encrypted1 =
            encrypt_bytes(plaintext, &key).unwrap();

        let encrypted2 =
            encrypt_bytes(plaintext, &key).unwrap();

        assert_ne!(
            encrypted1.nonce,
            encrypted2.nonce
        );

        assert_ne!(
            encrypted1.ciphertext,
            encrypted2.ciphertext
        );
    }

    #[test]
    fn creates_real_lock_file() {
        let input = "test_secret.txt";
        let audio = "test_audio.bin";
        let output = "test_secret.lock";

        fs::write(
            input,
            b"PhaseLock secret information",
        )
        .unwrap();

        fs::write(
            audio,
            b"fake audio key data",
        )
        .unwrap();

        encrypt_with_key(
            input,
            audio,
            output,
            Some("test-password"),
        )
        .unwrap();

        assert!(
            fs::metadata(output)
                .unwrap()
                .len() > 0
        );

        fs::remove_file(input).unwrap();
        fs::remove_file(audio).unwrap();
        fs::remove_file(output).unwrap();
    }
}