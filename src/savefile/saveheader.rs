use byteorder::{ByteOrder, BigEndian};

const STAGE_COUNT: usize = 42;
const WORLD_COUNT: usize = 10;

enum SaveFileRegion {
    NTSC,
    PAL,
    JPN,
    KOR,
    CHN,
    TW
}

pub struct SaveHeader {
    _region: SaveFileRegion,
    _last_selected_index: u8,
    _free_mode_play_count: [[u16; STAGE_COUNT]; WORLD_COUNT],
    _extra_modes_unlocked_worlds: u16 // flags for each world
}

impl SaveHeader {
    pub fn blank() -> Self {
        todo!()
    }

    pub fn from_bytes(input: &[u8]) -> Self {

        let region = match &input[4] {
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


        let extra_modes_unlocked_worlds = BigEndian::read_u16(&input[0x698..0x69A]);

        Self {
            _region: region,
            _last_selected_index: last_selected_index,
            _free_mode_play_count: free_mode_play_count,
            _extra_modes_unlocked_worlds: extra_modes_unlocked_worlds
        }
    }

    // pub fn to_bytes() -> Vec<u8> {
    //     todo!()
    // }

    // fn calculate_crc32() {
    //     todo!()
    // }
}
