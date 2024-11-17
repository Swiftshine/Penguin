use std::path::PathBuf;
use std::fs;


pub mod saveheader;
pub mod saveslot;
pub mod constants;
use saveheader::SaveHeader;
use saveslot::SaveSlot;

pub struct SaveFile {
    pub header: SaveHeader,
    pub save_slots: [SaveSlot; 6],
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
        let mut out = Vec::new();
        out.append(&mut self.header.to_bytes());

        for slot in self.save_slots.iter() {
            out.append(&mut slot.to_bytes());
        }

        out
    }
}
