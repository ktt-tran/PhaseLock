use std::path::PathBuf;
use std::sync::mpsc::Sender;
use egui::Ui;
use rfd::FileDialog;
use crate::worker::decrypt_worker::start_audio_decrypt;
use crate::worker::decrypt_worker::start_password_decrypt;
<<<<<<< HEAD
=======
use crate::utils::file_size::{format_size, total_size};
>>>>>>> v1.0.0

#[derive(Default)]
pub struct DecryptState {

    pub selected_lock: Option<PathBuf>,
    pub selected_audio: Option<PathBuf>,
    pub output_directory: Option<PathBuf>,
    pub password: String,
    pub show_password: bool,
    pub confirm_action: bool,

}


pub fn show(
    state: &mut DecryptState,
    status_message: &mut String,
    decrypt_job: &mut Option<std::thread::JoinHandle<std::io::Result<()>>>,
    decryption_running: &mut bool,
<<<<<<< HEAD
=======
    status_sender: &Sender<String>,
>>>>>>> v1.0.0
    ui: &mut Ui
) {

    ui.heading("Unlock File");

    ui.separator();

    // Select .lock file
    ui.label("Encrypted File:");

    if ui.button("Select .lock File").clicked(){

        if let Some(path)=FileDialog::new()
        .add_filter("Lock File", &["lock"])
        .pick_file(){

            state.selected_lock = Some(path);

        }
    }

    ui.label(format!(
        "File: {}",
        state.selected_lock
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "None selected".to_string())
    ));


    if state.selected_lock.is_some() {
        let total_bytes: u64 = state
            .selected_lock
            .iter()
            .map(|path| total_size(path))
            .sum();

        ui.label(format!(
            "Total size: {}",
            format_size(total_bytes)
        ));
    }

    ui.add_space(20.0);

    // Key upload
    ui.heading("Encryption Key");

    ui.label(
        "Upload the key used during encryption:"
    );

    if ui.button("Select Audio").clicked(){

        if let Some(path)=
            FileDialog::new()
            .add_filter("Audio Files",
                &[
                    "wav",
                    "mp3",
                    "flac",
                    "ogg",
                    "m4a"
                ])
            .pick_file()
        {

            state.selected_audio = Some(path);

        }
    }

    ui.label(format!(
        "Audio: {}",
        state.selected_audio
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "None selected".to_string())
    ));

    ui.add_space(20.0);

    // Password section
    ui.heading("Password");

    ui.label(
        "Enter password if one was added:"
    );

    crate::gui::components::password_input(
        ui,
        &mut state.password,
        &mut state.show_password
    );

    ui.add_space(30.0);

    if ui.button("🔒 Unlock").clicked(){
        if state.selected_lock.is_some() && (state.selected_audio.is_some() || !state.password.trim().is_empty()) {
            state.confirm_action = true;
        } else {
            *status_message = "Please select both a file and a unlocking method.".to_string();
<<<<<<< HEAD
            println!("Please select both a file and an unlocking method.");
=======
>>>>>>> v1.0.0
        }
    }

    if state.confirm_action {

        egui::Window::new(
            "Confirm Unlock"
        )
        .show(ui.ctx(), |ui| {

            ui.label(
                "Unlock selected file?"
            );

            ui.add_space(5.0);

            if ui.button("Select Directory:").clicked(){

                if let Some(path)=FileDialog::new()
                .pick_folder(){

                    state.output_directory = Some(path);

                }
            }

            ui.label(format!(
                "File: {}",
                state.output_directory
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "None selected".to_string())
            ));

            ui.add_space(5.0);

            ui.horizontal(|ui| {

                if ui.button("Cancel").clicked() {

                    state.confirm_action = false;

                }

                ui.add_enabled_ui(!*decryption_running, |ui| {
                    if ui.button("Unlock").clicked() && state.output_directory != None {
<<<<<<< HEAD
                        *status_message = "Unlocking...".to_string();
                        *decryption_running = true;
                        println!("Unlocking...");

                        let (Some(lock_file), Some(_output_directory)) = (&state.selected_lock, &state.output_directory) else { println!("Missing file detected; Second check."); return () };
=======
                        *decryption_running = true;

                        let (Some(lock_file), Some(_output_directory)) = (&state.selected_lock, &state.output_directory)
                            else { return; };
>>>>>>> v1.0.0

                        let mut output_path = _output_directory.clone();

                        if let Some(lock_name) = lock_file.file_stem().and_then(|s| s.to_str()) {
                            output_path = output_path.join(lock_name);
                        }

                        if let Some(audio_path) = &state.selected_audio {

                            *decrypt_job = Some(start_audio_decrypt(
                                lock_file.clone(),
                                audio_path.clone(),
                                output_path.clone(),
<<<<<<< HEAD
=======
                                status_sender.clone(),
>>>>>>> v1.0.0
                            ));
                        
                        } else if !state.password.trim().is_empty() {

                            *decrypt_job = Some(start_password_decrypt(
                                lock_file.clone(),
                                state.password.clone(),
                                output_path.clone(),
<<<<<<< HEAD
=======
                                status_sender.clone(),
>>>>>>> v1.0.0
                            ));

                        }
                        
                        state.password.clear();
                        state.password.shrink_to_fit();
                        state.selected_lock = None;
                        state.selected_audio = None;  
                        state.confirm_action = false;
                    }

                });

            });
        });
    }

}