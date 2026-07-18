use std::{
    fs,
    env,
    path::PathBuf,
    sync::mpsc::Sender,
};
use egui::Ui;
use rfd::FileDialog;
use crate::crypto::archive;
use crate::worker::viewer_worker::start_viewing_with_audio_decrypt;
use crate::worker::viewer_worker::start_viewing_with_password_decrypt;
use crate::worker::decrypt_worker::start_audio_decrypt;
use crate::worker::decrypt_worker::start_password_decrypt;

#[derive(Default)]
pub struct ViewerState {
    
    pub selected_lock: Option<PathBuf>,
    pub selected_audio: Option<PathBuf>,
    pub output_directory: Option<PathBuf>,
    pub password: String,
    pub show_password: bool,
    pub archive: Option<Vec<u8>>,
    pub file_list: Vec<String>,
    pub selected_file: Option<String>,
    pub confirm_action: bool,
    pub extraction: bool,

}

pub fn show(
    state: &mut ViewerState,
    status_message: &mut String,
    decrypt_job: &mut Option<std::thread::JoinHandle<std::io::Result<()>>>,
    viewer_job: &mut Option<std::thread::JoinHandle<std::io::Result<Vec<u8>>>>,
    decryption_running: &mut bool,
    viewer_running: &mut bool,
    status_sender: &Sender<String>,
    ui: &mut Ui
) {

    ui.heading("Preview");

    ui.separator();

    // Select .lock file
    ui.label("View Readable Files Without Extracting:");

    if ui.button("Select .lock File").clicked() {

        if let Some(file) = FileDialog::new()
            .add_filter("Lock File", &["lock"])
            .pick_file()
        {
            state.selected_lock = Some(file);
        }

    }

    ui.label(format!(
        "Selected: {}",
        state.selected_lock
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "None".to_string())
    ));

    ui.add_space(20.0);

    // Key input
    ui.heading("Encryption Key");

    ui.label(
        "Provide the key used during encryption:"
    );

    if ui.button("Select Audio File").clicked() {

        if let Some(file) = FileDialog::new()
            .pick_file()
        {
            state.selected_audio = Some(file);
        }

    }

    ui.label(format!(
        "Key: {}",
        state.selected_audio
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "None selected".to_string())
    ));

    ui.add_space(20.0);

    // Password
    ui.heading("Password");

    ui.label(
        "Enter password if required:"
    );

    crate::gui::components::password_input(
        ui,
        &mut state.password,
        &mut state.show_password
    );

    ui.add_space(30.0);


    if state.selected_lock.is_some() && (state.selected_audio.is_some() || !state.password.trim().is_empty()) {
        state.confirm_action = true;
    }

    // Actions
    if state.confirm_action {

        ui.heading("Actions");

        ui.horizontal(|ui| {

            ui.add_enabled_ui(!*viewer_running, |ui| {
                if ui.button("👁 View").clicked() && !*viewer_running {
                    *viewer_running = true;
                    state.archive = None;
                    state.file_list.clear();
                    state.selected_file = None;

                    let Some(lock_file) = &state.selected_lock
                            else { return; };

                    if let Some(audio_path) = &state.selected_audio {

                        *viewer_job = Some(start_viewing_with_audio_decrypt(
                            lock_file.clone(),
                            audio_path.clone(),
                            status_sender.clone(),
                        ));
                    
                    } else if !state.password.trim().is_empty() {

                        *viewer_job = Some(start_viewing_with_password_decrypt(
                            lock_file.clone(),
                            state.password.clone(),
                            status_sender.clone(),
                        ));

                    }

                    state.password.clear();
                    state.selected_lock = None;
                    state.selected_audio = None;  
                    state.confirm_action = false;
                }
            });

            if ui.button("📤 Extract").clicked() {
                state.extraction = true;
            }

            if state.extraction {
                egui::Window::new(
                    "Confirm Extract"
                )
                .show(ui.ctx(), |ui| {

                    ui.label(
                        "Extract selected file?"
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
                            state.extraction = false;

                        }

                        ui.add_enabled_ui(!*decryption_running, |ui| {
                            if ui.button("Extract").clicked() && state.output_directory != None {
                                *decryption_running = true;

                                let (Some(lock_file), Some(_output_directory)) = (&state.selected_lock, &state.output_directory)
                                    else { return; };

                                let mut output_path = _output_directory.clone();

                                if let Some(lock_name) = lock_file.file_stem().and_then(|s| s.to_str()) {
                                    output_path = output_path.join(lock_name);
                                }

                                if let Some(audio_path) = &state.selected_audio {

                                    *decrypt_job = Some(start_audio_decrypt(
                                        lock_file.clone(),
                                        audio_path.clone(),
                                        output_path.clone(),
                                        status_sender.clone(),
                                    ));
                                
                                } else if !state.password.trim().is_empty() {

                                    *decrypt_job = Some(start_password_decrypt(
                                        lock_file.clone(),
                                        state.password.clone(),
                                        output_path.clone(),
                                        status_sender.clone(),
                                    ));

                                }
                                
                                state.password.clear();
                                state.password.shrink_to_fit();
                                state.selected_lock = None;
                                state.selected_audio = None;  
                                state.confirm_action = false;
                                state.extraction = false;
                            }

                        });

                    });
                });
            }
        });
    }

    ui.add_space(10.0);

    ui.separator();

    ui.heading("Files");

    let files = state.file_list.clone();
    let temp_dir = env::temp_dir();

    for file in files.iter() {
        if ui.selectable_label(
                state.selected_file.as_deref() == Some(file.as_str()),
                file,
            ).clicked() {

            state.selected_file = Some(file.clone());
            let temp_path = temp_dir.join(file);

            if let Some(archive) = &state.archive {

                match archive::read_file_from_archive(archive, file) {

                    Ok(bytes) => {

                        let _ = fs::write(&temp_path, &bytes);
                        let _ = open::that(&temp_path);

                    }

                    Err(e) => { 
                        *status_message = format!("Failed to read file: {}", e); 
                    }

                }

            }
        }
    }

}