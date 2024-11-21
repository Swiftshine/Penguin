use eframe::egui;

use crate::savefile::{
    constants::*,
    saveslot::SaveSlot
};

pub struct SlotView {
    world_index_mushroom_house: usize,
    player_edit_index: usize,
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
            world_index_mushroom_house: 0,
            player_edit_index: 0
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, slot: &mut SaveSlot) {

        // row - game completion, world state
        ui.add_space(3.0);
        ui.horizontal(|ui|{
            // game completion

            ui.group(|ui|{
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

                    ui.vertical(|ui|{
                        ui.label("Hint movie unlocks");
                        egui::ScrollArea::vertical().show(ui, |ui|{
                            for i in 0..ACTUAL_HINT_MOVIE_COUNT {
                                ui.checkbox(
                                    &mut slot.hint_movie_bought[i],
                                    HINT_MOVIE_TITLES[i]
                                );
                            }
                        });
                    });
                });     
            });

            // world state
            ui.group(|ui|{
                ui.vertical(|ui|{
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
                        )
                        .show_ui(ui, |ui|{
                            for i in 0..=6 {
                                let val = match i {
                                    0 => StartingMushroomKind::None,
                                    1 => StartingMushroomKind::Star,
                                    2 => StartingMushroomKind::Item,
                                    3 => StartingMushroomKind::OneUp,
                                    4 => StartingMushroomKind::StarRescue,
                                    5 => StartingMushroomKind::ItemRescue,
                                    6 => StartingMushroomKind::OneUpRescue,
                                    _ => StartingMushroomKind::None
                                };

                                ui.selectable_value(
                                    &mut slot.starting_mushroom_house_type[self.world_index_mushroom_house],
                                    val,
                                    get_house_type_string(val)
                                )
                                .on_hover_text(
                                    "If there was not a toad house generated by the game, changing this value from None will not work."
                                );
                            }
                        });
                    });
                });
            });

        });
        
        ui.add_space(3.0);
        ui.horizontal(|ui|{
            // game state
            ui.group(|ui|{
                ui.vertical(|ui|{
                    ui.horizontal(|ui|{
                        ui.vertical(|ui|{
                            ui.label("Score");
                            ui.add(
                                egui::DragValue::new(&mut slot.ingame_score)
                                .speed(1)
                                .range(0..=MAX_SCORE)
                            );
                        });

                        ui.add_space(3.0);
                        ui.vertical(|ui|{
                            ui.label("Credits high score");
                            ui.add(
                                egui::DragValue::new(&mut slot.staff_credits_high_score)
                                .speed(1)
                                .range(0..=std::u16::MAX)
                            );
                        });
                    });

                    ui.add_space(3.0);
                    ui.group(|ui|{
                        ui.label("Item stock");
                        for i in 0..7 {
                            ui.horizontal(|ui|{
                                ui.add(
                                    egui::DragValue::new(&mut slot.item_stock[i])
                                    .speed(1)
                                    .range(0..=POWERUP_STOCK_MAX)          
                                );
                                ui.label(POWERUP_NAMES[i]);
                            });
                        }
                    });

                    
                });
            });
            
            // player information
            ui.group(|ui|{
                ui.horizontal(|ui|{
                    ui.vertical(|ui|{
                        
                        egui::ComboBox::from_label("Current world")
                            .selected_text(format!("World {}", slot.cur_world + 1))
                            .show_ui(ui, |ui|{
                                for i in 0..ACTUAL_WORLD_COUNT as u8 {
                                    ui.selectable_value(
                                        &mut slot.cur_world,
                                        i,
                                    format!("World {}", i + 1)
                                );
                            }
                        });
    
                        ui.label("Current subworld")
                        .on_hover_text("An example of a 'subworld' is the second half of World 3.");
                        ui.add(
                            egui::DragValue::new(&mut slot.cur_subworld)
                            .speed(1)
                            .range(0..=1)
                        );
    
                        ui.label("Current path node");
                        ui.add(
                            egui::DragValue::new(&mut slot.cur_path_node)
                            .speed(1)
                            .range(0..=std::u8::MAX)
                        );
                    });

                    ui.vertical(|ui|{
                        ui.group(|ui|{
                            egui::ComboBox::from_label("Selected player")
                            .selected_text(format!("Player {}", self.player_edit_index + 1))
                            .show_ui(ui, |ui|{
                                for i in 0..PLAYER_COUNT {
                                    ui.selectable_value(
                                        &mut self.player_edit_index,
                                        i,
                                        format!("Player {}", i + 1)
                                    );
                                }
                            });

                            egui::ComboBox::from_label("Character")
                            .selected_text(
                                match slot.player_character[self.player_edit_index] {
                                    PlayerCharacter::Mario => "Mario",
                                    PlayerCharacter::Luigi => "Luigi",
                                    PlayerCharacter::BlueToad => "Blue Toad",
                                    PlayerCharacter::YellowToad => "Yellow Toad"
                                }
                            )
                            .show_ui(ui, |ui|{
                                for i in 0..PLAYER_COUNT {
                                    ui.selectable_value(
                                        &mut slot.player_character[self.player_edit_index],
                                        match i {
                                            0 => PlayerCharacter::Mario,
                                            1 => PlayerCharacter::Luigi,
                                            2 => PlayerCharacter::BlueToad,
                                            3 => PlayerCharacter::YellowToad,
                                            _ => PlayerCharacter::Mario
                                        },
                                        PLAYER_NAMES[i]
                                    ).on_hover_text("Player 1 will always be Mario.");
                                }
                            });
                        });
                    });
                });
            });
        });

        
    }
}
