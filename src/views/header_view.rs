use eframe::egui;
use crate::savefile::{constants::SaveFileRegion, saveheader::SaveHeader};

pub struct HeaderView;

impl HeaderView {
    pub fn new() -> Self {
        Self
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, header: &mut SaveHeader) {
        egui::ComboBox::from_label("Region")
            .selected_text(
                match header.region {
                    SaveFileRegion::NTSC => "North America",
                    SaveFileRegion::PAL => "Europe/Australia",
                    SaveFileRegion::JPN => "Japan",
                    SaveFileRegion::KOR => "Korea",
                    SaveFileRegion::CHN => "China",
                    SaveFileRegion::TW => "Taiwan"
                }
            )
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut header.region, SaveFileRegion::NTSC, "North America");
                ui.selectable_value(&mut header.region, SaveFileRegion::PAL, "Europe/Australia");
                ui.selectable_value(&mut header.region, SaveFileRegion::JPN, "Japan");
                ui.selectable_value(&mut header.region, SaveFileRegion::KOR, "Korea");
                ui.selectable_value(&mut header.region, SaveFileRegion::CHN, "China");
                ui.selectable_value(&mut header.region, SaveFileRegion::TW, "Taiwan");

            }
        );

        egui::ComboBox::from_label("Last played save slot")
            .selected_text(String::from("Slot ") + (header.last_selected_index + 1).to_string().as_str())
            .show_ui(ui, |ui| {
                for i in 0..=2 {
                    let text = String::from("Slot") + &(i + 1).to_string();
                    ui.selectable_value(
                        &mut header.last_selected_index,
                        i,
                        text
                    );
                }
            }
        );

    }
}
