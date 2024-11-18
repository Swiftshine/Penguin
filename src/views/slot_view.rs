use eframe::egui;

use crate::savefile::{constants::{StartingMushroomKind, ACTUAL_WORLD_COUNT}, saveslot::SaveSlot};

pub struct SlotView {
    world_index_mushroom_house: usize
}

fn get_house_type_string(house_type: StartingMushroomKind) -> String {
    match house_type {
        StartingMushroomKind::None => "None",
        StartingMushroomKind::Star => "Star",
        StartingMushroomKind::Item => "Item",
        StartingMushroomKind::OneUp => "1-Up",
        StartingMushroomKind::StarRescue => "Star (Rescue)",
        StartingMushroomKind::ItemRescue => "Item (Rescue)",
        StartingMushroomKind::OneUpRescue => "1-Up (Rescue)",
    }.to_string()
}
impl SlotView {
    pub fn new() -> Self {
        Self {
            world_index_mushroom_house: 0
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, slot: &mut SaveSlot) {

        // row - game completion, world state
        ui.add_space(3.0);
        ui.label("Game completion");
        ui.horizontal(|ui|{
            // game completion
            egui::Frame::group(&ui.style())
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Completion flags");
                        // game completion flags
                        let labels = [
                            "Save empty",
                            "Final boss beaten",
                            "All goals",
                            "All star coins (W1-W8)",
                            "All star coins (W9)",
                            "Game completed",
                            "Super Guide triggered"  
                        ];
            
                        for i in 0..7 {
                            let mut is_checked = (slot.game_completion_flags & (1 << i)) != 0;
            
                            if ui.checkbox(&mut is_checked, labels[i]).changed() {
                                if is_checked {
                                    slot.game_completion_flags |= 1 << i;
                                } else {
                                    slot.game_completion_flags &= !(1 << i);
                                }
                            }
                        }
        
                    });
        
                    ui.vertical(|ui| {
                        ui.label("World unlocks");
                        // world unlocks
                        for i in 0..ACTUAL_WORLD_COUNT {
                            ui.checkbox(&mut slot.world_unlocked[i], format!("World {}", i + 1));
                        }
                    });
                });     
            });

            // world state
            egui::Frame::group(&ui.style())
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
            .show(ui, |ui|{
                ui.vertical(|ui|{
                    ui.label("World state");
                    // w3 switch
                    ui.checkbox(&mut slot.w3_switch_on, "World 3 switch on?");

                    // w5 vine reshuffle
                    ui.add_space(3.0);
                    ui.label("W5 vine reshuffle counter");
                    ui.add(
                        egui::DragValue::new(&mut slot.w5_vine_reshuffle_counter)
                        .speed(1)
                        .range(0..=255)
                    );

                    // mushroom house
                    ui.horizontal(|ui|{
                        egui::ComboBox::from_label("World")
                        .selected_text(
                            format!("World {}", self.world_index_mushroom_house + 1)
                        )
                        .show_ui(ui, |ui|{
                            for i in 0..ACTUAL_WORLD_COUNT {
                                ui.selectable_value(
                                    &mut self.world_index_mushroom_house,
                                    i,
                                    format!("World {}", i + 1)
                                );
                            }
                        });

                        egui::ComboBox::from_label("House type")
                        .selected_text(
                            get_house_type_string(slot.starting_mushroom_house_type[self.world_index_mushroom_house])
                        ).show_ui(ui, |ui|{

                            ui.selectable_value(
                                &mut slot.starting_mushroom_house_type[self.world_index_mushroom_house],
                                StartingMushroomKind::None,
                                "None"
                            );


                        });
                    });
                });
            });
        });
        

        
    }
}
