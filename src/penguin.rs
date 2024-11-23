use eframe::{self, egui, NativeOptions};
use egui::Button;
use std::path::PathBuf;
use std::env;
use rfd;
use std::fs;
use crate::savefile::SaveFile;
use crate::views::slot_view::SlotView;
use crate::views::{PenguinView, header_view::*};

pub struct PenguinApp {
    file_path: PathBuf,
    _settings: PenguinSettings,
    file: SaveFile,
    file_open: bool,
    current_view: PenguinView,
    header_view: HeaderView,
    current_slot_index: usize,
    slot_view: SlotView,
}

#[derive(Default)]
/// The settings that are saved to disk.
struct PenguinSettings {

}

fn get_slot_string(index: usize) -> String {
    match index {
        0..=2 => String::from("Save Slot ") + &format!("{}", index + 1),
        3..=5 => String::from("Quick Slot ") + &format!("{}", index - 2),
        _ => String::from("error")
    }
}

impl PenguinApp {
    fn new() -> Self {
        Self {
            file_path: env::current_dir().unwrap(),
            _settings: Default::default(),
            file: SaveFile::blank(),
            file_open: false,
            current_view: PenguinView::Header,
            header_view: HeaderView::new(),
            current_slot_index: 0,
            slot_view: SlotView::new()
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
                    
                    None => {}
                }
            }

            None => {}
        }
    }

    fn reopen(&mut self) {
        match SaveFile::from_path(&self.file_path) {
            Some(f) => self.file = f,
            None => {}
        }
    }

    fn try_save(&self, save_as: bool) {
        let mut empty = false;

        let path = if !save_as {
            self.file_path.clone()
        } else {
            match rfd::FileDialog::new()
                .add_filter("New Super Mario Bros. Wii save file", &["sav"])
                .set_can_create_directories(true)
                .save_file()
            {
                Some(p) => p,
                None => {
                    empty = true;
                    PathBuf::default()
                }
            }
        };

        // save file

        if empty {
            return;
        }

        match fs::exists(&path) {
            Ok(_b) => {
                match fs::write(&path, &self.file.to_bytes()) {
                    Ok(_) => {},
                    Err(_e) => {}
                }
            }

            Err(_e) => {}
        }

    }
}

impl eframe::App for PenguinApp {
    /// Called each time the UI needs to be repainted.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui|{
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

                if ui.add_enabled(self.file_open, Button::new("Refresh"))
                .on_hover_text("Reloads the save file. This can be used if the file was saved externally.")
                .clicked() {
                    self.reopen();
                    ui.close_menu();
                }


            });

        });


        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.file_open {
                ui.centered_and_justified(|ui|{
                    ui.label("Open a file.");
                });
            } else {
                ui.horizontal(|ui|{
                    ui.selectable_value(&mut self.current_view, PenguinView::Header, "Header");
                    ui.selectable_value(&mut self.current_view, PenguinView::SaveSlot, "Save Slots");
                });

                ui.separator();
                
                match self.current_view {
                    PenguinView::Header => {
                        self.header_view.show_ui(ui, &mut self.file.header);
                    }
        
                    PenguinView::SaveSlot => {
                        egui::ComboBox::from_label("Selected slot")
                            .selected_text(
                                get_slot_string(self.current_slot_index)   
                            )
                            .show_ui(ui, |ui|{
                                for i in 0..=5 {
                                    ui.selectable_value(&mut self.current_slot_index, i, get_slot_string(i));
                                }
                            });
                            
                        self.slot_view.show_ui(ui, &mut self.file.save_slots[self.current_slot_index]);
                    }
                }

            }
        });

    }

}
