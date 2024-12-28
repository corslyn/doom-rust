use crate::*;

impl Wad {
    /// Returns a vector of vertices
    pub fn get_vertices(&self, map_name: &str) -> Vec<Vertex> {
        let map_index = self.get_lump_index(map_name);

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

    /// Returns a vector of linedefs
    pub fn get_linedefs(&self, map_name: &str) -> Vec<Linedef> {
        let map_index = self.get_lump_index(map_name);

        let directory = self.read_directory();
        let linedefs_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "LINEDEFS")
            .unwrap();
        let linedefs_lump = &directory.lumps[map_index + 1 + linedefs_index];

        let mut linedefs = Vec::new();

        for i in 0..linedefs_lump.size / 14 {
            let offset = linedefs_lump.filepos + i as i32 * 14;
            let start = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let end = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            linedefs.push(Linedef { start, end });
        }
        linedefs
    }
}

impl Map {
    pub fn new(wad: &Wad, map_name: &str) -> Map {
        let vertexes = wad.get_vertices(map_name);
        let linedefs = wad.get_linedefs(map_name);
        Map {
            name: map_name.to_string(),
            vertexes,
            linedefs,
        }
    }
}
