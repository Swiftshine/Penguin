use std::path::PathBuf;
use std::fs;


mod saveheader;
mod saveslot;
use saveheader::SaveHeader;
use saveslot::SaveSlot;


pub struct SaveFile {
    _header: SaveHeader,
    _save_slots: [SaveSlot; 6],
}

impl SaveFile {
    pub fn blank() -> Self {
        Self {
            _header: SaveHeader::blank(),
            _save_slots: [
                SaveSlot::blank(),
                SaveSlot::blank(),
                SaveSlot::blank(),
                SaveSlot::blank(),
                SaveSlot::blank(),
                SaveSlot::blank()
            ]
        }
    }


    pub fn from_path(path: &PathBuf) -> Option<Self> {
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(_) => {
                return None;
            }
        };

        let magic = &bytes[..3];

        // validate magic
        if magic != b"SMN" {
            return None;
        }

        let header = SaveHeader::from_bytes(&bytes);

        let save_slots: [SaveSlot; 6] = [
            SaveSlot::from_bytes(&bytes, 0),
            SaveSlot::from_bytes(&bytes, 1),
            SaveSlot::from_bytes(&bytes, 2),
            SaveSlot::from_bytes(&bytes, 3),
            SaveSlot::from_bytes(&bytes, 4),
            SaveSlot::from_bytes(&bytes, 5),
        ];

        Some(Self {
            _header: header,
            _save_slots: save_slots
        })
    }
}
