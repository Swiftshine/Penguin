use std::path::PathBuf;
use std::fs;


mod saveheader;
mod saveslot;
mod constants;
use constants::SAVE_FILE_SIZE;
use saveheader::SaveHeader;
use saveslot::SaveSlot;

pub struct SaveFile {
    header: SaveHeader,
    save_slots: [SaveSlot; 6],
}

impl SaveFile {
    pub fn blank() -> Self {
        Self {
            header: SaveHeader::blank(),
            save_slots: [
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
            header,
            save_slots
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![0u8; SAVE_FILE_SIZE];
        out.extend_from_slice(&self.header.to_bytes());

        for slot in self.save_slots.iter() {
            out.extend_from_slice(&slot.to_bytes());
        }

        assert_eq!(out.capacity(), SAVE_FILE_SIZE);
        
        out
    }
}
