use std::fs;

use anyhow::Result;
use eframe::egui;
use json::{self, object};

#[derive(Clone, Copy, PartialEq)]
enum PenguinTheme {
    Dark,
    Light,
}

pub struct PenguinSettings {
    theme: PenguinTheme,
}

fn theme_to_string(theme: PenguinTheme) -> String {
    match theme {
        PenguinTheme::Dark => "Dark",
        PenguinTheme::Light => "Light",
    }
    .to_string()
}

impl PenguinSettings {
    pub fn default() -> Self {
        Self {
            theme: PenguinTheme::Dark,
        }
    }

    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Theme")
            .selected_text(theme_to_string(self.theme))
            .show_ui(ui, |ui| {
                let mut theme = self.theme;
                ui.selectable_value(&mut theme, PenguinTheme::Dark, "Dark");
                ui.selectable_value(&mut theme, PenguinTheme::Light, "Light");

                if theme != self.theme {
                    self.theme = theme;
                    self.update_theme(ui.ctx());
                }
            });

        if ui.button("Save settings").clicked() {
            self.save();
        }
    }

    pub fn update_theme(&self, ctx: &egui::Context) {
        match self.theme {
            PenguinTheme::Dark => ctx.set_theme(egui::Theme::Dark),
            PenguinTheme::Light => ctx.set_theme(egui::Theme::Light),
        }
    }

    pub fn load(&mut self) -> Result<()> {
        let exists = fs::exists("penguin_settings.json")?;

        if !exists {
            self.save();

            return Ok(());
        }

        let contents = fs::read_to_string("penguin_settings.json")?;
        let parsed = json::parse(&contents)?;

        self.theme = match parsed["theme"].as_str().unwrap() {
            "light" => PenguinTheme::Light,
            "dark" => PenguinTheme::Dark,
            _ => PenguinTheme::Dark,
        };

        Ok(())
    }

    pub fn save(&self) {
        let contents = object!(
            theme: theme_to_string(self.theme).to_lowercase()
        );

        let _ = fs::write("penguin_settings.json", contents.to_string());
    }
}
