use std::{
    fs,
    io,
    path::Path,
};

use chacha20poly1305::{
    aead::{Aead, AeadCore, Generate, Key, KeyInit},
    XChaCha20Poly1305,
    XNonce,
};

use crate::{
    audio::key::derive_audio_key,
    crypto::{
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
// The origin file will be encrypted and an audio file (and option
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

pub fn encrypt_file<P: AsRef<Path>, A: AsRef<Path>, O: AsRef<Path>>(
    input_path: P,
    audio_path: A,
    output_path: O,
    password: Option<&str>,
) -> io::Result<()> {

    let file_data = fs::read(input_path.as_ref())?;

    // Preserve original filename.
    let original_filename = match input_path.as_ref().file_name() {
        Some(fname) => fname.to_string_lossy().into_owned(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Inpute file has no filename",
            ));
        }
    };

    // Generate random key for file.
    let file_key = FileKey::generate();

    // Encrypt the actual file.
    let payload = encrypt_bytes(
        &file_data,
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
    let audio_key = derive_audio_key(audio_path)?;

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
        original_filename,
        audio_wrapped_key,
        password_data,
        payload
    };

    // Write the final .lock file.
    write_lock_file(
        output_path,
        &lock_file,
    )?;

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

        encrypt_file(
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