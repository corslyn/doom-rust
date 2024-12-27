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
pub struct Vertex {
    pub x_position: i16,
    pub y_position: i16,
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

        let numlumps = i32::from_le_bytes(self.read_n_bytes(4, 4).try_into().unwrap());
        let infotableofs = i32::from_le_bytes(self.read_n_bytes(8, 4).try_into().unwrap());

        WadHeader {
            wad_type,
            numlumps,
            infotableofs,
        }
    }

    /// Returns the vertex (x and y coordinates) at the given offset
    pub fn get_vertex(&self, offset: i32) -> Vertex {
        Vertex {
            x_position: i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap()),
            y_position: i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap()),
        }
    }

    /// Returns the lump index of the given lump name
    pub fn get_lump_index(&self, lump_name: &str) -> Option<usize> {
        self.read_directory()
            .lumps
            .iter()
            .position(|lump| lump.name.trim_end_matches('\0') == lump_name)
    }

    /// Takes an offset and read the next N bytes
    pub fn read_n_bytes(&self, offset: i32, bytes: usize) -> &[u8] {
        &self.data[offset as usize..offset as usize + bytes]
    }

    /// Returns all the lumps contained in the wad
    pub fn read_directory(&self) -> Directory {
        let header = self.read_header();
        let numlumps = header.numlumps;
        let mut lumps = vec![];
        for i in 0..numlumps {
            let offset = header.infotableofs + i * 16;
            let lump = Lump {
                filepos: i32::from_le_bytes(self.read_n_bytes(offset, 4).try_into().unwrap()),
                size: i32::from_le_bytes(self.read_n_bytes(offset + 4, 4).try_into().unwrap()),
                name: String::from_utf8(self.read_n_bytes(offset + 8, 8).to_vec()).unwrap(),
            };
            lumps.push(lump);
        }
        Directory { lumps }
    }

    /// Returns a vector of vertices
    pub fn get_vertices(&self, map_name: &str) -> Vec<Vertex> {
        let map_index = self.get_lump_index(map_name).unwrap();

        let directory = self.read_directory();
        let vertices_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "VERTEXES")
            .unwrap();
        let vertices_lump = &directory.lumps[map_index + 1 + vertices_index];

        let mut vertices = Vec::new();

        for i in 0..vertices_lump.size / 4 {
            let offset = vertices_lump.filepos + i as i32 * 4;
            let x = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let y = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            vertices.push(Vertex {
                x_position: x,
                y_position: y,
            });
        }
        vertices
    }
}
