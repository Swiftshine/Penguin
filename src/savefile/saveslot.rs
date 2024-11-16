use byteorder::{BigEndian, ByteOrder};

use crate::savefile::constants::*;
use crc32fast as crc32;

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
    world_unlocked: [bool; WORLD_COUNT],
    enemy_revival_count: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    staff_credits_high_score: u16,
    ingame_score: u32,
    stage_completion_flags: [[u32; STAGE_COUNT]; WORLD_COUNT],
    hint_movie_bought: [bool; HINT_MOVIE_COUNT],
    toad_rescue_level: [u8; WORLD_COUNT],
    enemy_subworld: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    enemy_pos_index: [[u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
    enemy_walk_direction: [[EnemyDirection; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
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
            world_unlocked: [false; WORLD_COUNT],
            enemy_revival_count: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            staff_credits_high_score: 0,
            ingame_score: 0,
            stage_completion_flags: [[0; STAGE_COUNT]; WORLD_COUNT],
            hint_movie_bought: [false; HINT_MOVIE_COUNT],
            toad_rescue_level: [0; WORLD_COUNT],
            enemy_subworld: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            enemy_pos_index: [[0; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            enemy_walk_direction: [[EnemyDirection::ToNextNode; AMBUSH_ENEMY_COUNT]; WORLD_COUNT],
            player_death_count: [[0; STAGE_COUNT]; WORLD_COUNT],
            player_death_count_w3_l4_switch: 0,
        }
    }

    pub fn from_bytes(input: &[u8], index: usize) -> Self  {
        let start_offset = HEADER_SIZE + (SAVE_SLOT_SIZE * index);

        let game_completion_flags = input[start_offset + 2];
        let cur_world = input[start_offset + 3];
        let cur_subworld = input[start_offset + 4];
        let cur_path_node = input[start_offset + 5];
        let w5_vine_reshuffle_counter = input[start_offset + 6];
        let w3_switch_on = input[start_offset + 7] != 0;

        let mut item_stock = [0u8; POWERUP_COUNT];
        for i in 0..POWERUP_COUNT {
            item_stock[i] = input[start_offset + 9 + i];
        }

        let mut starting_mushroom_house_type: [StartingMushroomKind; WORLD_COUNT] = [StartingMushroomKind::None; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            starting_mushroom_house_type[i] = match input[start_offset + 0x10 + i] {
                1 => StartingMushroomKind::Star,
                2 => StartingMushroomKind::Item,
                3 => StartingMushroomKind::OneUp,
                4 => StartingMushroomKind::StarRescue,
                5 => StartingMushroomKind::ItemRescue,
                6 => StartingMushroomKind::OneUpRescue,
                _ => StartingMushroomKind::None
            };
        }

        let mut player_continues = [0u8; PLAYER_COUNT];
        for i in 0..PLAYER_COUNT {
            player_continues[i] = input[start_offset + 0x1A + i];
        }

        let mut player_coins = [0u8; PLAYER_COUNT];
        for i in 0..PLAYER_COUNT {
            player_coins[i] = input[start_offset + 0x1E + i];
        }

        let mut player_lives = [0u8; PLAYER_COUNT];
        for i in 0..PLAYER_COUNT {
            player_lives[i] = input[start_offset + 0x22 + i];    
        }

        let mut player_spawn_flags = [0u8; PLAYER_COUNT];
        for i in 0..PLAYER_COUNT {
            player_spawn_flags[i] = input[start_offset + 0x26 + i];
        }

        let mut player_character = [
            PlayerCharacter::Mario,
            PlayerCharacter::Luigi,
            PlayerCharacter::YellowToad,
            PlayerCharacter::BlueToad
        ];

        for i in 0..PLAYER_COUNT {
            player_character[i] = match input[start_offset + 0x2A + i] {
                0 => PlayerCharacter::Mario,
                1 => PlayerCharacter::Luigi,
                2 => PlayerCharacter::BlueToad,
                3 => PlayerCharacter::YellowToad,

                // default to mario
                _ => PlayerCharacter::Mario,
            };
        }

        let mut player_powerup = [PlayerPowerup::None; PLAYER_COUNT];
        for i in 0..PLAYER_COUNT {
            player_powerup[i] = match input[start_offset + 0x2E + i] {
                0 => PlayerPowerup::None,
                1 => PlayerPowerup::Mushroom,
                2 => PlayerPowerup::FireFlower,
                3 => PlayerPowerup::MiniMushroom,
                4 => PlayerPowerup::PropellerMushroom,
                5 => PlayerPowerup::PenguinSuit,
                6 => PlayerPowerup::IceFlower,

                // default to none
                _ => PlayerPowerup::None,
            };
        }

        let mut world_unlocked = [false; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            world_unlocked[i] = input[start_offset + 0x32 + i] != 0;
        }

        let mut enemy_revival_count = [[0u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..AMBUSH_ENEMY_COUNT {
                enemy_revival_count[i][j] = input[start_offset + 0x3C + (i * AMBUSH_ENEMY_COUNT) + j];
            }
        }

        let staff_credits_high_score = BigEndian::read_u16(&input[start_offset + 0x66..start_offset + 0x68]);
        let ingame_score = BigEndian::read_u32(&input[start_offset + 0x68..start_offset + 0x6C]);
        
        let mut stage_completion_flags = [[0u32; STAGE_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                let start = start_offset + 0x6C + (i * STAGE_COUNT) + j;
                let end = start + 4;
                stage_completion_flags[i][j] = BigEndian::read_u32(&input[start..end]);
            }
        }

        let mut hint_movie_bought = [false; HINT_MOVIE_COUNT];
        for i in 0..HINT_MOVIE_COUNT {
            hint_movie_bought[i] = input[start_offset + 0x6FC + i] != 0;
        }

        let mut toad_rescue_level = [0u8; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            toad_rescue_level[i] = input[start_offset + 0x742 + i];
        }

        let mut enemy_subworld = [[0u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..AMBUSH_ENEMY_COUNT {
                enemy_subworld[i][j] = input[start_offset + 0x74C + (i * AMBUSH_ENEMY_COUNT) + j];
            }
        }

        let mut enemy_pos_index = [[0u8; AMBUSH_ENEMY_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..AMBUSH_ENEMY_COUNT {
                enemy_pos_index[i][j] = input[start_offset + 0x774 + (i * AMBUSH_ENEMY_COUNT) + j];
            }
        }

        let mut enemy_walk_direction = [[EnemyDirection::ToNextNode; AMBUSH_ENEMY_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..AMBUSH_ENEMY_COUNT {
                enemy_walk_direction[i][j] = match input[start_offset + 0x79C + (i * AMBUSH_ENEMY_COUNT) + j] {
                    0 => EnemyDirection::ToNextNode,
                    1 => EnemyDirection::ToPreviousNode,
                    2 => EnemyDirection::FirstTimeValue,

                    // default to next node
                    _ => EnemyDirection::ToNextNode,
                };
            }
        }

        let mut player_death_count = [[0u8; STAGE_COUNT]; WORLD_COUNT];
        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                player_death_count[i][j] = input[start_offset + 0x7C4 + (i * STAGE_COUNT) + j];
            }
        }

        let player_death_count_w3_l4_switch = input[start_offset + 0x968];
        
        Self {
            game_completion_flags,
            cur_world,
            cur_subworld,
            cur_path_node,
            w5_vine_reshuffle_counter,
            w3_switch_on,
            item_stock,
            starting_mushroom_house_type,
            player_continues,
            player_coins,
            player_lives,
            player_spawn_flags,
            player_character,
            player_powerup,
            world_unlocked,
            enemy_revival_count,
            staff_credits_high_score,
            ingame_score,
            stage_completion_flags,
            hint_movie_bought,
            toad_rescue_level,
            enemy_subworld,
            enemy_pos_index,
            enemy_walk_direction,
            player_death_count,
            player_death_count_w3_l4_switch
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![0u8; SAVE_SLOT_SIZE];
        
        // version
        out[0] = 0xE;

        out[2] = self.game_completion_flags;
        out[3] = self.cur_world;
        out[4] = self.cur_subworld;
        out[5] = self.cur_path_node;
        out[6] = self.w5_vine_reshuffle_counter;
        out[7] = if self.w3_switch_on { 1 } else { 0 };

        for i in 0..POWERUP_COUNT {
            out[9 + i] = self.item_stock[i];
        }

        
        for i in 0..PLAYER_COUNT {
            out[0x1A + i] = self.player_continues[i];
            out[0x1E + i] = self.player_coins[i];
            out[0x22 + i] = self.player_lives[i];
            out[0x26 + i] = self.player_spawn_flags[i];
            
            out[0x2A + i] = match self.player_character[i] {
                PlayerCharacter::Mario => 0,
                PlayerCharacter::Luigi => 1,
                PlayerCharacter::BlueToad => 2,
                PlayerCharacter::YellowToad => 3,
            };

            out[0x2E + i] = match self.player_powerup[i] {
                PlayerPowerup::None => 0,
                PlayerPowerup::Mushroom => 1,
                PlayerPowerup::FireFlower => 2,
                PlayerPowerup::MiniMushroom => 3,
                PlayerPowerup::PropellerMushroom => 4,
                PlayerPowerup::PenguinSuit => 5,
                PlayerPowerup::IceFlower => 6,
            };
        }
        
        for i in 0..WORLD_COUNT {
            out[0x10 + i] = match self.starting_mushroom_house_type[i] {
                StartingMushroomKind::None => 0,
                StartingMushroomKind::Star => 1,
                StartingMushroomKind::Item => 2,
                StartingMushroomKind::OneUp => 3,
                StartingMushroomKind::StarRescue => 4,
                StartingMushroomKind::ItemRescue => 5,
                StartingMushroomKind::OneUpRescue => 6,
            };

            out[0x32 + i] = if self.world_unlocked[i] { 1 } else { 0 };

            for j in 0..AMBUSH_ENEMY_COUNT {
                let offs = (i * AMBUSH_ENEMY_COUNT) + j;

                out[0x3C + offs] = self.enemy_revival_count[i][j];
                out[0x74C + offs] = self.enemy_subworld[i][j];
                out[0x79C + offs] = match self.enemy_walk_direction[i][j] {
                    EnemyDirection::ToNextNode => 0,
                    EnemyDirection::ToPreviousNode => 1,
                    EnemyDirection::FirstTimeValue => 2,
                };
            }

            for j in 0..STAGE_COUNT {
                let offs = (i * STAGE_COUNT) + j;
                BigEndian::write_u32(
                    &mut out[0x6C + offs..0x6C + offs + 4],
                    self.stage_completion_flags[i][j]
                );

                out[0x7C4 + offs] = self.player_death_count[i][j];
            }
        }
        
        BigEndian::write_u16(&mut out[0x66..0x68], self.staff_credits_high_score);
        BigEndian::write_u32(&mut out[0x68..0x6C], self.ingame_score);
        
        for i in 0..HINT_MOVIE_COUNT {
            out[0x6FC + i] = if self.hint_movie_bought[i] { 1 } else { 0 };
        }
        
        out[0x968] = self.player_death_count_w3_l4_switch;

        let crc = crc32::hash(&out[..SAVE_SLOT_SIZE - 4]);
        
        BigEndian::write_u32(
            &mut out[0x97C..],
            crc
        );

        out
    }
}
