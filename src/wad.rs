use byteorder::LittleEndian;
use std::fs;
#[derive(Debug)]

pub struct Wad {
    pub data: Vec<u8>,
}

pub struct WadHeader {
    pub wad_type: String,
    pub numlumps: i32,
    pub infotableofs: i32,
}

impl Wad {
    pub fn new(file_path: std::path::PathBuf) -> Wad {
        let data = fs::read(file_path).expect("Unable to read file");
        Wad { data }
    }

    pub fn read_header(&self) -> WadHeader {
        let wad_type = match std::str::from_utf8(&self.data[..4]) {
            Ok("IWAD") => "IWAD".to_string(),
            Ok("PWAD") => "PWAD".to_string(),
            _ => {
                eprintln!("Unknown WAD format");
                std::process::exit(1);
            }
        };

        let numlumps = i32::from_le_bytes(self.read_4_bytes(4).try_into().unwrap());
        let infotableofs = i32::from_le_bytes(self.read_4_bytes(8).try_into().unwrap());

        WadHeader {
            wad_type,
            numlumps,
            infotableofs,
        }
    }

    pub fn read_4_bytes(&self, offset: u32) -> Vec<u8> {
        self.data[offset as usize..offset as usize + 4].to_vec()
    }
}
