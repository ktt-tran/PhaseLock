use std::path::PathBuf;
use egui::Ui;
use rfd::FileDialog;
use crate::crypto::decrypt::decrypt_file_with_audio;
use crate::crypto::decrypt::decrypt_file_with_password;
use crate::file::reader::read_lock_file;

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
        if (state.selected_lock != None && state.selected_audio != None) || (state.selected_lock != None && !state.password.trim().is_empty()) {
            state.confirm_action = true;
        } else {
            *status_message = "Please select both a file and an audio key.".to_string();
            println!("Please select both a file and an audio key.");
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

                
                if ui.button("Unlock").clicked() && state.output_directory != None {
                    *status_message = "Unlocking...".to_string();
                    println!("Unlocking...");
                    
                    // Unlock file using the input audio key.
                    if let (Some(lock_file), Some(audio_path), Some(output_directory)) = (&state.selected_lock, &state.selected_audio, &state.output_directory) {
                        
                        let lock_data = read_lock_file(lock_file);

                        let output_directory = output_directory.join(
                            &lock_data.unwrap().original_filename
                        );

                        let result_audio = decrypt_file_with_audio(
                            lock_file,
                            audio_path,
                            &output_directory,
                        );

                        match result_audio {
                            Ok(()) => {
                                *status_message = format!(
                                    "Unlock Successful: {}",
                                    output_directory.display()
                                );

                                println!(
                                    "Unlock Successful: {}",
                                    output_directory.display()
                                );

                                state.password.clear();
                                state.password.shrink_to_fit();

                                state.selected_lock = None;
                                state.selected_audio = None;
                            }

                            _ => if !state.password.trim().is_empty() {
                                
                                let result_password = decrypt_file_with_password(
                                    lock_file,
                                    state.password.trim(),
                                    &output_directory,
                                );

                                match result_password {
                                    Ok(()) => {
                                        *status_message = format!(
                                            "Unlock Successful: {}",
                                            output_directory.display()
                                        );

                                        println!(
                                            "Unlock Successful: {}",
                                            output_directory.display()
                                        );

                                        state.password.clear();
                                        state.password.shrink_to_fit();

                                        state.selected_lock = None;
                                        state.selected_audio = None;
                                    }

                                    Err(error) => {
                                        *status_message = format!("Unlock Failed: {error}");
                                        println!("Unlock Failed: {error}");
                                    }
                                }
                            }
                        }
                    } else if let (Some(lock_file), Some(output_directory)) = (&state.selected_lock, &state.output_directory) && !state.password.trim().is_empty() {

                        let lock_data = read_lock_file(lock_file);

                        let output_directory = output_directory.join(
                            &lock_data.unwrap().original_filename
                        );

                        let result_password = decrypt_file_with_password(
                            lock_file,
                            state.password.trim(),
                            &output_directory,
                        );

                        match result_password {
                            Ok(()) => {
                                *status_message = format!(
                                    "   Unlock Successful: {}",
                                    output_directory.display()
                                );

                                println!(
                                    "Unlock Successful: {}",
                                    output_directory.display()
                                );

                                state.password.clear();
                                state.password.shrink_to_fit();

                                state.selected_lock = None;
                                state.selected_audio = None;
                            }

                            Err(error) => {
                                *status_message = format!("Unlock Failed: {error}");
                                println!("Unlock Failed: {error}");
                            }
                        }

                    } else {
                        println!("Neccessry credentials were incorrect or not provided.")
                    }
                    
                    state.confirm_action = false;
                }
            });
        });
    }

}