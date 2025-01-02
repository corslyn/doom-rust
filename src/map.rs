use crate::data_types::{Map, Wad};

pub enum LumpIndex {
    THINGS = 1,
    LINEDEFS,
    SIDEDEFS,
    VERTEXES,
    SEAGS,
    SSECTORS,
    NODES,
    SECTORS,
    REJECT,
    BLOCKMAP,
}

impl Map {
    pub fn new(wad: &Wad, map_name: &str) -> Map {
        Map {
            map_name: map_name.to_string(),
            vertices: wad.get_vertices(map_name),
            linedefs: wad.get_linedefs(map_name),
        }
    }
}
