use eframe::egui;
use crate::gui;
use std::sync::mpsc::{Receiver, Sender};
use crate::crypto::archive;

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
    pub encrypt_job: Option<std::thread::JoinHandle<std::io::Result<()>>>,
    pub decrypt_job: Option<std::thread::JoinHandle<std::io::Result<()>>>,
    pub viewer_job: Option<std::thread::JoinHandle<std::io::Result<Vec<u8>>>>,
    pub encryption_running: bool,
    pub decryption_running: bool,
    pub viewer_running: bool,
    pub status_sender: Sender<String>,
    pub status_receiver: Receiver<String>,

}


impl PhaseLockApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        cc.egui_ctx.set_visuals(
            egui::Visuals::dark()
        );

        let (status_sender, status_receiver) = std::sync::mpsc::channel();
        
        Self {

            current_page: Page::Home,
            status_message:"Ready".to_string(),
            encrypt_state:gui::encrypt::EncryptState::default(),
            decrypt_state:gui::unlock::DecryptState::default(),
            viewer_state:gui::viewer::ViewerState::default(),
            encrypt_job: None,
            decrypt_job: None,
            viewer_job: None,
            encryption_running:false,
            decryption_running:false,
            viewer_running:false,
            status_sender,
            status_receiver,

        }
    }
}


impl eframe::App for PhaseLockApp {

    fn update(
        &mut self,
        ctx:&egui::Context,
        _frame:&mut eframe::Frame
    ){

        // Update status bar on progress.
        while let Ok(message) = self.status_receiver.try_recv() {
            self.status_message = message;
        }

        // Poll encryption job.
        if self.encryption_running {
            if let Some(job) = &self.encrypt_job {
                if job.is_finished() {

                    let job = self.encrypt_job.take().unwrap();

                    match job.join().unwrap() {
                        Ok(()) => {
                            self.status_message = "Encryption Successful!".to_string();
                        }

                        Err(error) => {
                            self.status_message = format!("Encryption Failed: {}", error);
                        }
                    }

                    self.encryption_running = false;
                }
            }
        }

        // Poll decryption job.
        if self.decryption_running {
            if let Some(job) = &self.decrypt_job {
                if job.is_finished() {

                    let job = self.decrypt_job.take().unwrap();

                    match job.join().unwrap() {
                        Ok(()) => {
                            self.status_message = "Unlock Successful!".to_string();
                        }

                        Err(error) => {
                            self.status_message = format!("Unlock Failed: {}", error);
                        }
                    }

                    self.decryption_running = false;
                }
            }
        }

        if self.viewer_running {
            if let Some(job) = self.viewer_job.take() {
                if job.is_finished() {
                    match job.join().unwrap() {
                        Ok(archive) => {
                            match archive::list_archive_files(&archive) {

                                Ok(files) => {

                                    self.viewer_state.archive = Some(archive);
                                    self.viewer_state.file_list = files;
                                    self.status_message = "Viewer ready!".to_string();

                                }

                                Err(e) => {

                                    self.status_message = format!(
                                        "Failed to read archive: {}", e
                                    );

                                }

                            }
                        }

                        Err(e) => {
                            self.status_message = format!(
                                "Failed to unlock: {}", e
                            );
                        }
                    }

                    self.viewer_running = false;
                    
                } else {
                    self.viewer_job = Some(job);
                }
            }
        }

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

            else if ui.button("👁 Previewer").clicked(){
                self.current_page = Page::Viewer;
            }
        });


        if self.encryption_running || self.decryption_running {
            ctx.request_repaint();
        }

        egui::CentralPanel::default()
        .show(ctx, |ui|{


            match self.current_page {

                Page::Home => home::show(ui),

                Page::Encrypt =>
                    encrypt::show(
                        &mut self.encrypt_state,
                        &mut self.status_message,
                        &mut self.encrypt_job,
                        &mut self.encryption_running,
                        &self.status_sender,
                        ui
                    ),

                Page::Unlock =>
                    unlock::show(
                        &mut self.decrypt_state,
                        &mut self.status_message,
                        &mut self.decrypt_job,
                        &mut self.decryption_running,
                        &self.status_sender,
                        ui
                    ),

                Page::Viewer =>
                    viewer::show(
                        &mut self.viewer_state,
                        &mut self.status_message,
                        &mut self.decrypt_job,
                        &mut self.viewer_job,
                        &mut self.decryption_running,
                        &mut self.viewer_running,
                        &self.status_sender,
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