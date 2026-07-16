use std::path::PathBuf;
use egui::Ui;
use rfd::FileDialog;
use crate::crypto::encrypt::encrypt_with_key;


#[derive(Default)]
pub struct EncryptState {

    pub selected_items: Vec<PathBuf>,
    pub selected_audio: Option<PathBuf>,
    pub output_directory: Option<PathBuf>,
    pub output_name: String,
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

    ui.label(
        "Select Files or a Folder to Encrypt"
    );

    // Populate vec of files and/or folders to be encrypted.
    ui.horizontal(|ui| {
        if ui.button("Add File(s)").clicked() {
            if let Some(files) = FileDialog::new().pick_files() {
                    for file in files {
                    if !state.selected_items.contains(&file) {
                        state.selected_items.push(file);
                    }
                }
            }
        }

        if ui.button("Add Folder").clicked() {
            if let Some(folder) = FileDialog::new().pick_folder() {
                if !state.selected_items.contains(&folder) {
                    state.selected_items.push(folder);
                }
            }
        }

        if ui.button("Clear Selection").clicked() {
            state.selected_items.clear();
        }
    });

    // Scroll box to view all files selected.
    if state.selected_items.is_empty() {
        ui.label("No items selected.");
    } else {
        ui.label(format!(
            "{} item(s) selected",
            state.selected_items.len()
        ));

        egui::ScrollArea::vertical()
        .max_height(100.0)
        .show(ui, |ui| {
            for path in &state.selected_items {
                ui.label(format!("• {}", path.file_name().unwrap_or_default().to_string_lossy()));
            }
        });
    }

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
        if state.selected_items.is_empty() || state.selected_audio.is_none() {
            *status_message = "Please select both a file or folder and an audio key.".to_string();
            println!("Please select both a file or folder and an audio key.");
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
                "Encrypt selected item?"
            );

            ui.add_space(5.0);

            if ui.button("Select Directory:").clicked(){

                if let Some(path)=FileDialog::new()
                .pick_folder(){

                    state.output_directory = Some(path);

                }
            }

            ui.label(format!(
                "Output Directory: {}",
                state.output_directory
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "None selected".to_string())
            ));

            ui.add_space(5.0);

            ui.label("Lock File Name:");

            ui.add(
                egui::TextEdit::singleline(&mut state.output_name)
                    .hint_text("MyEncrypted")
            );

            ui.add_space(5.0);

            ui.horizontal(|ui| {

                if ui.button("Cancel").clicked() {

                    state.confirm_action = false;

                }

                // Update status bar to indicate the encryption is in progress.
                if ui.button("Encrypt").clicked() && state.output_directory.is_some() {
                    *status_message = "Encrypting...".to_string();
                    println!("Encrypting...");

                    let (Some(audio_path), Some(output_directory)) = (&state.selected_audio, &state.output_directory)
                        else { println!("Missing file detected; Second check."); return () };
            
                    let output_name = if state.output_name.trim().is_empty() {
                        "MyEncrypted"
                    } else {
                        state.output_name.trim()
                    };

                    let output_path = output_directory.join(output_name).with_extension("lock");

                    let password = if state.use_password && !state.password.trim().is_empty()
                    {
                        Some(state.password.as_str())
                    } else {
                        None
                    };

                    let result = encrypt_with_key(
                        &state.selected_items,
                        audio_path,
                        &output_path,
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
                                let mut delete_errors = Vec::new();

                                // Attempts to delete all uploaded file after encrypting them.
                                for item in &state.selected_items {
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

                                if delete_errors.is_empty() {
                                    *status_message =
                                        "Original files deleted successfully".to_string();
                                } else {
                                    *status_message = format!(
                                        "Some files could not be deleted:\n{}",
                                        delete_errors.join("\n")
                                    );
                                }
                            }
                        }

                        Err(error) => {
                            *status_message = format!("Encryption Failed: {error}");
                            println!("Encryption Failed: {error}");
                        }
                    }

                    state.password.clear();
                    state.password.shrink_to_fit();

                    state.selected_items = vec![];
                    state.selected_audio = None;
                    state.output_directory = None;
                    state.confirm_action = false;
                }
            });
        });
    }

}