use crate::wad::Wad;

pub struct Map {
    pub name: String,
    // pub things: Option<Vec<Thing>>,
    // pub linedefs: Option<Vec<Linedef>>,
    // pub sidedefs: Option<Vec<Sidedef>>,
    pub vertexes: Vec<Vertex>,
    //  pub segs: Option<Vec<Seg>>,
    //  pub ssectors: Option<Vec<SSubsector>>,
    //  pub nodes: Option<Vec<Node>>,
    //   pub sectors: Option<Vec<Sector>>,
    //  pub reject: Option<Vec<u8>>,
    //  pub blockmap: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Vertex {
    pub x_position: i16,
    pub y_position: i16,
}

impl Wad {
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

impl Map {
    pub fn new(wad: &Wad, map_name: &str) -> Map {
        let vertexes = wad.get_vertices(map_name);
        Map {
            name: map_name.to_string(),
            vertexes,
        }
    }
}
