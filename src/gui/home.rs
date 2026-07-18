use egui::Ui;


pub fn show(ui:&mut Ui){

    ui.vertical_centered(|ui|{

        ui.add_space(50.0);

        ui.heading(
            "🔒 PhaseLock"
        );

        ui.add_space(15.0);

        ui.label(
            "Secure audio-based file encryption"
        );

        ui.add_space(10.0);

        ui.label(
            "Protect files using an audio key and optional password."
        );

    });

}