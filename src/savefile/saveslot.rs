use crate::savefile::constants::*;

const SAVE_SLOT_SIZE: usize = 0x980;

pub struct SaveSlot {
    game_completion_flags: u8,
    cur_world: u8,
    cur_subworld: u8,
    cur_path_node: u8,
    w5_vine_reshuffle_counter: u8,
    w3_switch_on: bool,
    item_stock: [u8; POWERUP_COUNT],
    starting_mushroom_house_type: [StartingMushroomKind; WORLD_COUNT],
    player_continues: [u8; PLAYER_COUNT],
    player_coins: [u8; PLAYER_COUNT],
    player_lives: [u8; PLAYER_COUNT],
    player_spawn_flags: [u8; PLAYER_COUNT],
    player_character: [PlayerCharacter; PLAYER_COUNT],
    player_powerup: [PlayerPowerup; PLAYER_COUNT],
    world_completion_flags: [u8; WORLD_COUNT],
    enemy_revival_count: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    staff_credits_high_score: u16,
    ingame_score: u32,
    stage_completion_flags: [[u32; STAGE_COUNT]; WORLD_COUNT],
    hint_movie_bought: [bool; HINT_MOVIE_COUNT],
    toad_rescue_level: [u8; WORLD_COUNT],
    enemy_subworld: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    enemy_pos_index: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    enemy_walk_direction: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    player_death_count: [[u8; STAGE_COUNT]; WORLD_COUNT],
    player_death_count_w3_l4_switch: u8,
}

impl SaveSlot {
    pub fn blank() -> Self {
        Self {
            game_completion_flags: 0,
            cur_world: 0,
            cur_subworld: 0,
            cur_path_node: 0,
            w5_vine_reshuffle_counter: 0,
            w3_switch_on: false,
            item_stock: [0; POWERUP_COUNT],
            starting_mushroom_house_type: [StartingMushroomKind::None; WORLD_COUNT],
            player_continues: [0; PLAYER_COUNT],
            player_coins: [0; PLAYER_COUNT],
            player_lives: [0; PLAYER_COUNT],
            player_spawn_flags: [0; PLAYER_COUNT],
            player_character: [
                PlayerCharacter::Mario,
                PlayerCharacter::Luigi,
                PlayerCharacter::YellowToad,
                PlayerCharacter::BlueToad,
            ],
            player_powerup: [PlayerPowerup::None; PLAYER_COUNT],
            world_completion_flags: [0; WORLD_COUNT],
            enemy_revival_count: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            staff_credits_high_score: 0,
            ingame_score: 0,
            stage_completion_flags: [[0; STAGE_COUNT]; WORLD_COUNT],
            hint_movie_bought: [false; HINT_MOVIE_COUNT],
            toad_rescue_level: [0; WORLD_COUNT],
            enemy_subworld: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            enemy_pos_index: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            enemy_walk_direction: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            player_death_count: [[0; STAGE_COUNT]; WORLD_COUNT],
            player_death_count_w3_l4_switch: 0,
        }
    }

    pub fn from_bytes(_input: &[u8], _index: usize) -> Self  {
        todo!()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
