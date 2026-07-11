use egui::Ui;


pub fn password_input(
    ui:&mut Ui,
    password:&mut String,
    visible:&mut bool
){

    ui.horizontal(|ui|{


        if *visible {

            ui.text_edit_singleline(password);

        }
        else {

            ui.add(
                egui::TextEdit::singleline(password)
                .password(true)
            );

        }


        if ui.button("👁").clicked(){

            *visible = !*visible;

        }

    });

}