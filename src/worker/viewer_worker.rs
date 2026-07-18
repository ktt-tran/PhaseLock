use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
    sync::mpsc::Sender,
};

use crate::crypto::decrypt::{
    decrypt_with_audio,
    decrypt_with_password,
};

/// Starts viewing using audio-key.
pub fn start_viewing_with_audio_decrypt(
    lock_file: PathBuf,
    audio_path: PathBuf,
    status_sender: Sender<String>,
) -> JoinHandle<std::io::Result<Vec<u8>>> {

    thread::spawn(move || {

        decrypt_with_audio(
            &lock_file,
            &audio_path,
            &status_sender,
        )

    })
}

/// Starts viewing using password.
pub fn start_viewing_with_password_decrypt(
    lock_file: PathBuf,
    password: String,
    status_sender: Sender<String>,
) -> JoinHandle<std::io::Result<Vec<u8>>> {

    thread::spawn(move || {

        decrypt_with_password(
            &lock_file,
            &password,
            &status_sender,
        )

    })
}