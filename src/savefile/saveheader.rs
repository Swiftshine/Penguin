use byteorder::{ByteOrder, BigEndian};
use crc32fast as crc32;

use crate::savefile::constants::*;

pub struct SaveHeader {
    pub region: SaveFileRegion,
    pub last_selected_index: u8,
    pub free_mode_play_count: [[u16; STAGE_COUNT]; WORLD_COUNT],
    pub coin_battle_play_count: [[u16; STAGE_COUNT]; WORLD_COUNT],
    pub extra_modes_unlocked_worlds: u16, // flags for each world
}

impl SaveHeader {
    pub fn blank() -> Self {
        Self {
            region: SaveFileRegion::NTSC,
            last_selected_index: 0,
            free_mode_play_count: [[0; STAGE_COUNT]; WORLD_COUNT],
            coin_battle_play_count: [[0; STAGE_COUNT]; WORLD_COUNT],
            extra_modes_unlocked_worlds: 0
        }
    }

    pub fn from_bytes(input: &[u8]) -> Self {
        let region = match &input[3] {
            b'E' => SaveFileRegion::NTSC,
            b'P' => SaveFileRegion::PAL,
            b'J' => SaveFileRegion::JPN,
            b'K' => SaveFileRegion::KOR,
            b'C' => SaveFileRegion::CHN,
            b'W' => SaveFileRegion::TW,

            _ => {
                // default to NTSC
                SaveFileRegion::NTSC
            }
        };

        let last_selected_index = input[0x6];

        let mut free_mode_play_count: [[u16; STAGE_COUNT]; WORLD_COUNT] = [[0; STAGE_COUNT]; WORLD_COUNT];

        let mut offs = 0x8;

        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                free_mode_play_count[i][j] = BigEndian::read_u16(&input[offs..offs + 2]);
                offs += 2;
            }
        }

        let mut coin_battle_play_count: [[u16; STAGE_COUNT]; WORLD_COUNT] = [[0; STAGE_COUNT]; WORLD_COUNT];

        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                coin_battle_play_count[i][j] = BigEndian::read_u16(&input[offs..offs + 2]);
                offs += 2;
            }
        }


        let extra_modes_unlocked_worlds = BigEndian::read_u16(&input[0x698..0x69A]);

        Self {
            region,
            last_selected_index,
            free_mode_play_count,
            coin_battle_play_count,
            extra_modes_unlocked_worlds,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = [0u8; HEADER_SIZE];
        
        // magic
        out[0] = b'S';
        out[1] = b'M';
        out[2] = b'N';
        out[3] = match self.region {
            SaveFileRegion::NTSC => b'E',
            SaveFileRegion::PAL  => b'P',
            SaveFileRegion::JPN  => b'J',
            SaveFileRegion::KOR  => b'K',
            SaveFileRegion::CHN  => b'C',
            SaveFileRegion::TW   => b'W'
        };

        // version - 0x0E00.
        out[0x4] = 0xE;

        // last selected save file
        out[0x6] = self.last_selected_index;

        let mut offs = 8;

        // play count of each level in free for all/free mode
        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                BigEndian::write_u16(
                    &mut out[offs..offs + 2],
                    self.free_mode_play_count[i][j]
                );

                offs += 2;
            }
        }

        // play count of each level in coin battle
        for i in 0..WORLD_COUNT {
            for j in 0..STAGE_COUNT {
                BigEndian::write_u16(
                    &mut out[offs..offs + 2],
                    self.coin_battle_play_count[i][j]
                );

                offs += 2;
            }
        }

        // unlocked worlds flags
        BigEndian::write_u16(
            &mut out[0x698..0x69A],
            self.extra_modes_unlocked_worlds
        );

        // crc32 is calculated excluding the magic
        let crc = crc32::hash(&out[4..0x69C]);

        BigEndian::write_u32(
            &mut out[0x69C..0x6A0],
            crc
        );

        out.to_vec()
    }
}
