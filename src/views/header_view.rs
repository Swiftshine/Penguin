use eframe::egui;
use crate::savefile::{constants::{SaveFileRegion, ACTUAL_WORLD_COUNT}, saveheader::SaveHeader};

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

            });

        egui::ComboBox::from_label("Last played save slot")
            .selected_text(String::from("Slot ") + &format!("{}", header.last_selected_index + 1))
            .show_ui(ui, |ui| {
                for i in 0..=2 {
                    let text = String::from("Slot") + &format!("{}", i + 1);
                    ui.selectable_value(
                        &mut header.last_selected_index,
                        i,
                        text
                    );
                }
            });
        
        // the play counts in the extra modes are omitted for now
        
        ui.add_space(3.0);
        ui.label("Unlocked worlds in extra game modes");
        egui::Frame::group(&ui.style())
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .show(ui, |ui|{
                egui::ScrollArea::vertical().show(ui, |ui|{
                    for i in 0..ACTUAL_WORLD_COUNT {
                        let mut is_checked = (header.extra_modes_unlocked_worlds & (1 << i)) != 0;
        
                        if ui.checkbox(&mut is_checked, format!("World {}", i + 1)).changed() {
                            if is_checked {
                                header.extra_modes_unlocked_worlds |= 1 << i;
                            } else {
                                header.extra_modes_unlocked_worlds &= !(1 << i);
                            }
                        }
                    }
                });
            });
    }
}
