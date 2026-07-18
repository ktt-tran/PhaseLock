use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
<<<<<<< HEAD
=======
    sync::mpsc::Sender,
>>>>>>> v1.0.0
};

use crate::crypto::encrypt::encrypt_with_key;

/// Starts encryption on a background thread.
pub fn start_encrypt(
    selected_items: Vec<PathBuf>,
    audio_path: PathBuf,
    output_path: PathBuf,
    password: Option<String>,
    delete_original: bool,
<<<<<<< HEAD
=======
    status_sender: Sender<String>,
>>>>>>> v1.0.0
) -> JoinHandle<std::io::Result<()>> {

    thread::spawn(move || {

        encrypt_with_key(
            &selected_items,
            &audio_path,
            &output_path,
            password.as_deref(),
<<<<<<< HEAD
=======
            &status_sender,
>>>>>>> v1.0.0
        )?;

        if delete_original {
            let mut delete_errors = Vec::new();

            // Attempts to delete all uploaded file after encrypting them.
            for item in &selected_items {
                let result = if item.is_file() {
                    std::fs::remove_file(item)
                } else if item.is_dir() {
                    std::fs::remove_dir_all(item)
                } else {
                    continue;
                };

                if let Err(e) = result {
                    delete_errors.push(format!(
                        "{}: {}",
                        item.display(),
                        e
                    ));
                }
            }

            if !delete_errors.is_empty() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    delete_errors.join("\n"),
                ));
            }
        }

        Ok(())
    })
}