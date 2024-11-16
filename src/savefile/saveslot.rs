use crate::savefile::constants::*;

const SAVE_SLOT_SIZE: usize = 0x980;

pub struct SaveSlot {
    _game_completion: u8, // flags
    _cur_world: u8,
    _cur_subworld: u8,
    _cur_path_node: u8,
    _w5_vine_reshuffle_counter: u8,
    _w3_switch_on: bool,
    _item_stock: [u8; POWERUP_COUNT],
    _starting_mushroom_house_type: [u8; WORLD_COUNT],
    _player_continues: [u8; PLAYER_COUNT],
    _player_coins: [u8; PLAYER_COUNT],
    _player_lives: [u8; PLAYER_COUNT],
    _player_span_flags: [u8; PLAYER_COUNT],
    _player_character: [PlayerCharacter; PLAYER_COUNT],
    _player_powerup: [PlayerPowerup; PLAYER_COUNT],
    _world_completion_flags: [u8; WORLD_COUNT],
    _enemy_revival_count: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    _staff_credits_high_score: u16,
    _ingame_score: u32,
    _stage_completion_flags: [[u32; STAGE_COUNT]; WORLD_COUNT],
    _hint_movie_bought: [bool; HINT_MOVIE_COUNT],
    _toad_rescue_level: [u8; WORLD_COUNT],
    _enemy_subworld: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    _enemy_pos_index: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    _enemy_walk_direction: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    _player_death_count: [[u8; STAGE_COUNT]; WORLD_COUNT],
    _player_death_count_w3_l4_switch: u8,
}

impl SaveSlot {
    pub fn blank() -> Self {
        todo!()
    }

    pub fn from_bytes(_input: &[u8], _index: usize) -> Self  {
        todo!()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
