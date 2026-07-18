use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
<<<<<<< HEAD
};

use crate::crypto::decrypt::{
    decrypt_with_audio,
    decrypt_with_password,
=======
    sync::mpsc::Sender,
};

use crate::{
    crypto::{
        decrypt::{decrypt_and_extract_with_audio, decrypt_and_extract_with_password,},
    }
>>>>>>> v1.0.0
};

/// Starts audio-key decryption.
pub fn start_audio_decrypt(
    lock_file: PathBuf,
    audio_path: PathBuf,
    output_directory: PathBuf,
<<<<<<< HEAD
=======
    status_sender: Sender<String>,
>>>>>>> v1.0.0
) -> JoinHandle<std::io::Result<()>> {

    thread::spawn(move || {

<<<<<<< HEAD
        decrypt_with_audio(
            &lock_file,
            &audio_path,
            &output_directory,
=======
        decrypt_and_extract_with_audio(
            &lock_file,
            &audio_path,
            &output_directory,
            &status_sender,
>>>>>>> v1.0.0
        )

    })
}

/// Starts password decryption.
pub fn start_password_decrypt(
    lock_file: PathBuf,
    password: String,
    output_directory: PathBuf,
<<<<<<< HEAD
=======
    status_sender: Sender<String>,
>>>>>>> v1.0.0
) -> JoinHandle<std::io::Result<()>> {

    thread::spawn(move || {

<<<<<<< HEAD
        decrypt_with_password(
            &lock_file,
            &password,
            &output_directory,
        )

=======
        decrypt_and_extract_with_password(
            &lock_file,
            &password,
            &output_directory,
            &status_sender,
        )
        
>>>>>>> v1.0.0
    })
}