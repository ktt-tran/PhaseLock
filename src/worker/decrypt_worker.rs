use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
    sync::mpsc::Sender,
};

use crate::{
    crypto::{
        decrypt::{decrypt_and_extract_with_audio, decrypt_and_extract_with_password,},
    }
};

/// Starts audio-key decryption.
pub fn start_audio_decrypt(
    lock_file: PathBuf,
    audio_path: PathBuf,
    output_directory: PathBuf,
    status_sender: Sender<String>,
) -> JoinHandle<std::io::Result<()>> {

    thread::spawn(move || {

        decrypt_and_extract_with_audio(
            &lock_file,
            &audio_path,
            &output_directory,
            &status_sender,
        )

    })
}

/// Starts password decryption.
pub fn start_password_decrypt(
    lock_file: PathBuf,
    password: String,
    output_directory: PathBuf,
    status_sender: Sender<String>,
) -> JoinHandle<std::io::Result<()>> {

    thread::spawn(move || {

        decrypt_and_extract_with_password(
            &lock_file,
            &password,
            &output_directory,
            &status_sender,
        )
        
    })
}