use std::fs;
#[derive(Debug)]

pub struct Wad {
    pub data: Vec<u8>,
}

pub struct WadHeader {
    /// Wad type: IWAD or PWAD
    pub wad_type: String,

    /// Amount of lumps
    pub numlumps: i32,

    /// Offset where the lumps begins
    pub infotableofs: i32,
}

#[derive(Debug)]

/// Represents one lump of the wad
pub struct Lump {
    pub filepos: i32,
    pub size: i32,
    pub name: String,
}

#[derive(Debug)]
/// Contains all the lumps of the wad
pub struct Directory {
    pub lumps: Vec<Lump>,
}

impl Wad {
    pub fn new(file_path: std::path::PathBuf) -> Wad {
        let data = fs::read(file_path).expect("Unable to read file");
        Wad { data }
    }

    /// Reads the wad file and returns its header
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

    /// Takes an offset and read the next 4 bytes
    pub fn read_4_bytes(&self, offset: i32) -> Vec<u8> {
        self.data[offset as usize..offset as usize + 4].to_vec()
    }

    /// Takes an offset and read the next 8 bytes
    pub fn read_8_bytes(&self, offset: i32) -> Vec<u8> {
        self.data[offset as usize..offset as usize + 8].to_vec()
    }

    /// Returns all the lumps contained in the wad
    pub fn read_directory(&self) -> Directory {
        let header = self.read_header();
        let numlumps = header.numlumps;
        let mut lumps = vec![];
        for i in 0..numlumps {
            let offset = header.infotableofs + i * 16;
            let lump = Lump {
                filepos: i32::from_le_bytes(self.read_4_bytes(offset).try_into().unwrap()),
                size: i32::from_le_bytes(self.read_4_bytes(offset + 4).try_into().unwrap()),
                name: String::from_utf8(self.read_8_bytes(offset + 8)).unwrap(),
            };
            lumps.push(lump);
        }
        Directory { lumps }
    }
}
