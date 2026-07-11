use egui::Ui;
use rfd::FileDialog;


#[derive(Default)]
pub struct EncryptState {

    pub selected_file: String,
    pub selected_audio: String,
    pub password: String,
    pub use_password: bool,
    pub show_password: bool,
    pub confirm_action: bool

}


pub fn show(
    state: &mut EncryptState,
    ui: &mut Ui
){

    ui.heading("Encrypt File");

    ui.separator();

    if ui.button("Select File").clicked(){

        if let Some(path)=FileDialog::new().pick_file(){

            state.selected_file = path.display().to_string();

        }
    }

    ui.label(
        format!(
            "File: {}",
            state.selected_file
        )
    );

    ui.add_space(20.0);

    ui.heading("Audio Key Source");

    ui.label(
        "Primary method: Audio File"
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

            state.selected_audio =
                path.display().to_string();

        }

    }

    ui.label(
        format!(
            "Audio: {}",
            state.selected_audio
        )
    );

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

    ui.add_space(20.0);

    if ui.button("🔒 Encrypt").clicked(){

        state.confirm_action = true;

    }

    if state.confirm_action {

        egui::Window::new(
            "Confirm Encryption"
        )
        .show(ui.ctx(), |ui| {

            ui.label(
                "Encrypt selected file?"
            );

            ui.horizontal(|ui| {

                if ui.button("Cancel").clicked() {

                    state.confirm_action = false;

                }

                if ui.button("Encrypt").clicked() {

                    println!("Encrypting...");

                    state.confirm_action = false;


                    // Temporary:
                    // simulate successful encryption
                    state.password.clear();
                    state.password.shrink_to_fit();

                }

            });

        });

    }
}