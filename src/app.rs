use eframe::egui;
use crate::gui;

use crate::gui::{
    home,
    encrypt,
    unlock,
    viewer,
};


#[derive(Clone, Copy)]
pub enum Page {

    Home,
    Encrypt,
    Unlock,
    Viewer,

}


pub struct PhaseLockApp {

    pub current_page: Page,
    pub encrypt_state: gui::encrypt::EncryptState,
    pub decrypt_state: gui::unlock::DecryptState,
    pub viewer_state: gui::viewer::ViewerState,
    pub status_message: String,

}


impl PhaseLockApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        cc.egui_ctx.set_visuals(
            egui::Visuals::dark()
        );

        Self {

            current_page: Page::Home,

            status_message:
                "Ready".to_string(),

            encrypt_state:
                gui::encrypt::EncryptState::default(),

            decrypt_state:
                gui::unlock::DecryptState::default(),

            viewer_state:
                gui::viewer::ViewerState::default(),

        }
    }
}


impl eframe::App for PhaseLockApp {

    fn update(
        &mut self,
        ctx:&egui::Context,
        _frame:&mut eframe::Frame
    ){

        egui::SidePanel::left("sidebar")
        .show(ctx, |ui|{

            ui.heading("🔒 PhaseLock");
            ui.separator();

            if ui.button("🏠 Home").clicked(){
                self.current_page = Page::Home;
            }

            else if ui.button("🔐 Encrypt").clicked(){
                self.current_page = Page::Encrypt;
            }

            else if ui.button("🔓 Unlock").clicked(){
                self.current_page = Page::Unlock;
            }

            else if ui.button("👁 Secure Viewer").clicked(){
                self.current_page = Page::Viewer;
            }
        });


        egui::CentralPanel::default()
        .show(ctx, |ui|{


            match self.current_page {

                Page::Home => home::show(ui),

                Page::Encrypt =>
                    encrypt::show(
                        &mut self.encrypt_state,
                        &mut self.status_message,
                        ui
                    ),

                Page::Unlock =>
                    unlock::show(
                        &mut self.decrypt_state,
                        &mut self.status_message,
                        ui
                    ),

                Page::Viewer =>
                    viewer::show(
                        &mut self.viewer_state,
                        ui
                    ),

            }
        });

        egui::TopBottomPanel::bottom("status")
        .show(ctx, |ui| {

            ui.horizontal(|ui| {

                ui.label("Status:");

                ui.label(
                    &self.status_message
                );

            });

        });

    }

}