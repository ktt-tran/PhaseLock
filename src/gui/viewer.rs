use egui::Ui;
use rfd::FileDialog;


#[derive(Default)]
pub struct ViewerState {

    pub selected_lock: String,
    pub selected_audio: String,
    pub password: String,
    pub show_password: bool,

}


pub fn show(
    state: &mut ViewerState,
    ui: &mut Ui
) {

    ui.heading("Secure Viewer");

    ui.separator();

    // Select .lock file
    ui.label("Encrypted File:");

    if ui.button("Select .lock File").clicked() {

        if let Some(file) = FileDialog::new()
            .add_filter("Lock File", &["lock"])
            .pick_file()
        {
            state.selected_lock =
                file.display().to_string();
        }

    }

    ui.label(
        format!(
            "Selected: {}",
            state.selected_lock
        )
    );

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
            state.selected_audio =
                file.display().to_string();
        }

    }

    ui.label(
        format!(
            "Key: {}",
            state.selected_audio
        )
    );

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

    // Actions
    if !state.selected_lock.is_empty()
    {

        ui.heading("Actions");

        ui.horizontal(|ui| {

            if ui.button("👁 View").clicked() {

                println!("Authenticating and opening preview...");

            }

            if ui.button("📤 Extract").clicked() {

                println!("Authenticating and extracting...");

            }
        });

    }

}