use std::path::PathBuf;
use std::sync::mpsc::Sender;
use egui::Ui;
use rfd::FileDialog;
use crate::worker::encrypt_worker::start_encrypt;
use crate::utils::file_size::{format_size, total_size, GB};


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
    encrypt_job: &mut Option<std::thread::JoinHandle<std::io::Result<()>>>,
    encryption_running: &mut bool,
    status_sender: &Sender<String>,
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

        if ui.button("Clear").clicked() {
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

        let total_bytes: u64 = state
            .selected_items
            .iter()
            .map(|path| total_size(path))
            .sum();

        if total_bytes > GB as u64 {
            ui.label(format!(
                "Total size: {} (Larger data can be slow to encrypt)",
                format_size(total_bytes)
            ));
        } else {
            ui.label(format!(
                "Total size: {}",
                format_size(total_bytes)
            ));
        }
    }

    ui.add_space(20.0);

    ui.heading("Audio Key Source");

    ui.label(
        "Audio File"
    );

    if ui.button("Select Audio File").clicked(){

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

                ui.add_enabled_ui(!*encryption_running, |ui| {
                    // Update status bar to indicate the encryption is in progress.
                    if ui.button("Encrypt").clicked() && state.output_directory.is_some() {
                        *encryption_running = true;

                        let (Some(audio_path), Some(output_directory)) = (&state.selected_audio, &state.output_directory)
                            else { return; };
                
                        let output_name = if state.output_name.trim().is_empty() {
                            "MyEncrypted"
                        } else {
                            state.output_name.trim()
                        };

                        let output_path = output_directory.join(output_name).with_extension("lock");

                        let password = if state.use_password && !state.password.trim().is_empty()
                        {
                            Some(state.password.clone())
                        } else {
                            None
                        };

                        // Starting a new thread need to transfer ownership.
                        *encrypt_job = Some(start_encrypt(
                            state.selected_items.clone(),
                            audio_path.clone(),
                            output_path.clone(),
                            password,
                            state.delete_original,
                            status_sender.clone(),
                        ));

                        state.password.clear();
                        state.password.shrink_to_fit();
                        state.selected_items = vec![];
                        state.selected_audio = None;
                        state.output_directory = None;
                        state.confirm_action = false;
                    }
                });
            });
        });
    }

}