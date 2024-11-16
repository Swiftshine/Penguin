use eframe::{self, egui, NativeOptions};
use egui::Button;
use std::path::PathBuf;
use std::env;
use rfd;
use std::fs;

use crate::savefile::SaveFile;

pub struct PenguinApp {
    file_path: PathBuf,
    _settings: PenguinSettings,
    file: SaveFile,
    file_open: bool,
}

#[derive(Default)]
/// The settings that are saved to disk.
struct PenguinSettings {

}

impl PenguinApp {
    fn new() -> Self {
        Self {
            file_path: env::current_dir().unwrap(),
            _settings: Default::default(),
            file: SaveFile::blank(),
            file_open: false,
        }
    }

    /// Runs the application
    pub fn run() -> Result<(), eframe::Error> {
        eframe::run_native(
            "Penguin",
            NativeOptions::default(),
            Box::new(|_cc| {
                Ok(Box::<PenguinApp>::from(
                    PenguinApp::new()
                ))
            })
        )
    }

    fn try_open(&mut self) {
        let path = rfd::FileDialog::new()
            .add_filter("New Super Mario Bros. Wii save file", &["sav"])
            .pick_file();
        
        match path {
            Some(p) => {
                self.file_path = p;

                match SaveFile::from_path(&self.file_path) {
                    Some(f) => {
                        self.file_open = true;
                        self.file = f;
                    },
                    
                    None => {
                        // tell the user the file was invalid
                        todo!()
                    }
                }
            }

            None => {}
        }
    }

    fn try_save(&self, save_as: bool) {
        let path = if !save_as {
            self.file_path.clone()
        } else {
            match rfd::FileDialog::new()
                .add_filter("New Super Mario Bros. Wii save file", &["sav"])
                .save_file()
            {
                Some(p) => p,
                None => {
                    todo!()
                }
            }
        };

        // save file

        match fs::write(&path, &self.file.to_bytes()) {
            Ok(_) => {},
            Err(_e) => {
                todo!()
            }
        }
    }
}

impl eframe::App for PenguinApp {
    /// Called each time the UI needs to be repainted.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
        .show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    self.try_open();
                    ui.close_menu();
                }
                
                if ui.add_enabled(self.file_open, Button::new("Save"))
                    .clicked()
                {
                    self.try_save(false);
                    ui.close_menu();
                }
                
                if ui.add_enabled(self.file_open, Button::new("Save As"))
                    .clicked()
                {
                    self.try_save(true);
                    ui.close_menu();
                }
            });

        });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("central panel");
        });
    }
}
