use std::path::PathBuf;
use egui::Ui;
use rfd::FileDialog;
use crate::crypto::encrypt::encrypt_file;


#[derive(Default)]
pub struct EncryptState {

    pub selected_file: Option<PathBuf>,
    pub selected_audio: Option<PathBuf>,
    pub output_directory: Option<PathBuf>,
    pub password: String,
    pub use_password: bool,
    pub show_password: bool,
    pub delete_original: bool,
    pub confirm_action: bool,

}


pub fn show(
    state: &mut EncryptState,
    status_message: &mut String,
    ui: &mut Ui
){

    ui.heading("Encrypt File");

    ui.separator();

    if ui.button("Select File").clicked(){

        if let Some(path)=FileDialog::new().pick_file(){

            state.selected_file = Some(path);

        }
    }

    ui.label(format!(
        "File: {}",
        state.selected_file
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "None selected".to_string())
    ));

    ui.add_space(20.0);

    ui.heading("Audio Key Source");

    ui.label(
        "Audio File"
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

    ui.checkbox(
        &mut state.use_password,
        "Add Password Protection"
    );

    if state.use_password {

        crate::gui::components::password_input(
            ui,
            &mut state.password,
            &mut state.show_password
        );

    }

    ui.add_space(5.0);

    ui.checkbox(
        &mut state.delete_original,
        "Delete Original After Encryption"
    );

    ui.add_space(20.0);

    if ui.button("🔒 Encrypt").clicked(){
        if state.selected_file == None || state.selected_audio == None {
            *status_message = "Please select both a file and an audio key.".to_string();
            println!("Please select both a file and an audio key.");
        } else {
            state.confirm_action = true;
        }
    }

    if state.confirm_action {

        egui::Window::new(
            "Confirm Encryption"
        )
        .show(ui.ctx(), |ui| {

            ui.label(
                "Encrypt selected file?"
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

                // Update status bar to indicate the encryption is in progress.
                if ui.button("Encrypt").clicked() && state.output_directory != None {
                    *status_message = "Encrypting...".to_string();
                    println!("Encrypting...");
  
                    let (Some(input_path), Some(audio_path), Some(output_directory)) = (&state.selected_file, &state.selected_audio, &state.output_directory) else { println!("Missing file detected; Second check."); return () };
                    
                    let output_directory = output_directory.join(
                        input_path.file_name().unwrap_or_default()
                    ).with_extension("lock");

                    let password = if state.use_password && !state.password.trim().is_empty()
                    {
                        Some(state.password.as_str())
                    } else {
                        None
                    };

                    let result = encrypt_file(
                        input_path,
                        audio_path,
                        &output_directory,
                        password,
                    );

                    match result {
                        Ok(()) => {
                            *status_message = format!(
                                "Encryption Successful: {}",
                                output_directory.display()
                            );

                            println!(
                                "Encryption Successful: {}",
                                output_directory.display()
                            );

                            if state.delete_original {
                                match std::fs::remove_file(input_path) {
                                    Ok(()) => {
                                        *status_message = "Encryption Successful, Original File was Deleted".to_string();
                                        println!("Encryption Successful, Original File was Deleted");
                                    }

                                    Err(error) => {
                                        *status_message = format!("Encryption Successful, Original File was Not Deleted: {error}");
                                        println!("Encryption Successful, Original File was Not Deleted: {error}");
                                    }
                                }
                            }

                            state.password.clear();
                            state.password.shrink_to_fit();

                            state.selected_file = None;
                            state.selected_audio = None;
                        }

                        Err(error) => {
                            *status_message = format!("Encryption Failed: {error}");
                            println!("Encryption Failed: {error}");
                        }
                    }
                }
            });
        });
    }

}