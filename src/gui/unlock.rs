use egui::Ui;
use rfd::FileDialog;


#[derive(Default)]
pub struct DecryptState {

    pub selected_lock: String,
    pub selected_audio: String,
    pub password: String,
    pub show_password: bool,

}


pub fn show(
    state: &mut DecryptState,
    ui: &mut Ui
) {

    ui.heading("Unlock File");

    ui.separator();

    // Select .lock file
    ui.label("Encrypted File:");

    if ui.button("Select .lock File").clicked() {

        if let Some(file) = FileDialog::new()
            .add_filter("Lock File", &["lock"])
            .pick_file()
        {
            state.selected_lock = file.display().to_string();
        }

    }

    ui.label(
        format!("Selected: {}", state.selected_lock)
    );


    ui.add_space(20.0);

    // Key upload
    ui.heading("Encryption Key");

    ui.label(
        "Upload the key used during encryption:"
    );

    if ui.button("Select Audio File").clicked() {

        if let Some(file) = FileDialog::new()
            .pick_file()
        {
            state.selected_audio = file.display().to_string();
        }
    }

    ui.label(
        format!("Key: {}", state.selected_audio)
    );

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

    if ui.button("🔓 Unlock").clicked() {

        println!("Unlocking file...");

    }

}