use std::{
    io,
    path::Path,
};

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
    XNonce,
};

use crate::{
    audio::key::derive_audio_key,
    crypto::{
        archive,
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

pub fn decrypt_with_audio<L: AsRef<Path>, A: AsRef<Path>, O: AsRef<Path>>(
    lock_path: L,
    audio_path: A,
    output_directory: O,
) -> io::Result<()> {

    // Read and parse the .lock file.
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

    println!("Reading .lock...");
    println!("Unwrapping key...");
    println!("Decrypting payload...");

    // Decrypt the actual file contents
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

    println!("Payload decrypted.");
    println!("Archive size: {}", decrypted.len());
    
    // Restore archive of files/folders.
    archive::extract_archive(
        &decrypted,
        output_directory.as_ref()
    )?;

    Ok(())
}

pub fn decrypt_with_password<L: AsRef<Path>, O: AsRef<Path>>(
    lock_path: L,
    password: &str,
    output_directory: O,
) -> io::Result<()> {

    // Read the .lock file
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

        // Restore archive of files/folders.
        archive::extract_archive(
            &decrypted,
            output_directory.as_ref()
        )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::crypto::encrypt::encrypt_bytes;
    use crate::crypto::key::FileKey;
    use crate::crypto::encrypt::wrap_file_key;
    use crate::crypto::encrypt::encrypt_file;

    #[test]
    fn decrypt_restores_original_data() {
        let key = [42u8; 32];
        let plaintext = b"PhaseLock secret file";

        let encrypted =
            encrypt_bytes(plaintext, &key).unwrap();

        let decrypted =
            decrypt_bytes(&encrypted, &key).unwrap();

        assert_eq!(
            decrypted,
            plaintext
        );
    }

    #[test]
    fn wrong_key_fails() {
        let correct_key = [42u8; 32];
        let wrong_key = [99u8; 32];

        let plaintext = b"Secret information";

        let encrypted =
            encrypt_bytes(
                plaintext,
                &correct_key,
            )
            .unwrap();

        let result =
            decrypt_bytes(
                &encrypted,
                &wrong_key,
            );

        assert!(result.is_err());
    }

    #[test]
    fn modified_ciphertext_fails() {
        let key = [42u8; 32];

        let mut encrypted =
            encrypt_bytes(
                b"Important data",
                &key,
            )
            .unwrap();

        encrypted.ciphertext[0] ^= 1;

        let result =
            decrypt_bytes(
                &encrypted,
                &key,
            );

        assert!(result.is_err());
    }

    #[test]
    fn file_key_can_be_wrapped_and_unwrapped() {
        let file_key = FileKey::generate();

        let wrapping_key = [55u8; 32];

        let wrapped =
            wrap_file_key(
                &file_key,
                &wrapping_key,
            )
            .unwrap();

        let recovered =
            unwrap_file_key(
                &wrapped,
                &wrapping_key,
            )
            .unwrap();

        assert_eq!(
            file_key.as_bytes(),
            recovered.as_bytes()
        );
    }

    #[test]
    fn wrong_wrapping_key_cannot_recover_file_key() {
        let file_key = FileKey::generate();

        let correct_key = [55u8; 32];
        let wrong_key = [99u8; 32];

        let wrapped =
            wrap_file_key(
                &file_key,
                &correct_key,
            )
            .unwrap();

        let result =
            unwrap_file_key(
                &wrapped,
                &wrong_key,
            );

        assert!(result.is_err());
    }

    #[test]
    fn encrypt_and_decrypt_with_audio() {
        let input = "test_original.txt";
        let audio = "test_audio.bin";
        let lock = "test_output.lock";
        let restored = "test_restored.txt";

        let original_data =
            b"PhaseLock end-to-end encryption test";

        fs::write(
            input,
            original_data,
        )
        .unwrap();

        fs::write(
            audio,
            b"test audio key data",
        )
        .unwrap();

        encrypt_file(
            input,
            audio,
            lock,
            None,
        )
        .unwrap();

        decrypt_with_audio(
            lock,
            audio,
            restored,
        )
        .unwrap();

        let restored_data =
            fs::read(restored).unwrap();

        assert_eq!(
            restored_data,
            original_data
        );

        fs::remove_file(input).unwrap();
        fs::remove_file(audio).unwrap();
        fs::remove_file(lock).unwrap();
        fs::remove_file(restored).unwrap();
    }

    #[test]
    fn wrong_audio_cannot_unlock_file() {
        let input = "test_original_wrong.txt";
        let correct_audio = "test_correct_audio.bin";
        let wrong_audio = "test_wrong_audio.bin";
        let lock = "test_wrong_audio.lock";
        let output = "should_not_exist.txt";

        fs::write(
            input,
            b"Secret data",
        )
        .unwrap();

        fs::write(
            correct_audio,
            b"correct audio data",
        )
        .unwrap();

        fs::write(
            wrong_audio,
            b"wrong audio data",
        )
        .unwrap();

        encrypt_file(
            input,
            correct_audio,
            lock,
            None,
        )
        .unwrap();

        let result =
            decrypt_with_audio(
                lock,
                wrong_audio,
                output,
            );

        assert!(result.is_err());

        fs::remove_file(input).unwrap();
        fs::remove_file(correct_audio).unwrap();
        fs::remove_file(wrong_audio).unwrap();
        fs::remove_file(lock).unwrap();

        if Path::new(output).exists() {
            fs::remove_file(output).unwrap();
        }
    }

    #[test]
    fn encrypt_and_decrypt_with_password() {
        let input = "test_password_original.txt";
        let audio = "test_password_audio.bin";
        let lock = "test_password.lock";
        let restored = "test_password_restored.txt";

        let original_data =
            b"PhaseLock password unlock test";

        fs::write(
            input,
            original_data,
        )
        .unwrap();

        fs::write(
            audio,
            b"audio key data",
        )
        .unwrap();

        encrypt_file(
            input,
            audio,
            lock,
            Some("correct-password"),
        )
        .unwrap();

        decrypt_with_password(
            lock,
            "correct-password",
            restored,
        )
        .unwrap();

        let restored_data =
            fs::read(restored).unwrap();

        assert_eq!(
            restored_data,
            original_data
        );

        fs::remove_file(input).unwrap();
        fs::remove_file(audio).unwrap();
        fs::remove_file(lock).unwrap();
        fs::remove_file(restored).unwrap();
    }

    #[test]
    fn wrong_password_cannot_unlock_file() {
        let input = "test_wrong_password.txt";
        let audio = "test_wrong_password_audio.bin";
        let lock = "test_wrong_password.lock";
        let output = "wrong_password_output.txt";

        fs::write(
            input,
            b"Secret PhaseLock data",
        )
        .unwrap();

        fs::write(
            audio,
            b"audio key data",
        )
        .unwrap();

        encrypt_file(
            input,
            audio,
            lock,
            Some("correct-password"),
        )
        .unwrap();

        let result =
            decrypt_with_password(
                lock,
                "wrong-password",
                output,
            );

        assert!(result.is_err());

        fs::remove_file(input).unwrap();
        fs::remove_file(audio).unwrap();
        fs::remove_file(lock).unwrap();

        if Path::new(output).exists() {
            fs::remove_file(output).unwrap();
        }
    }
}