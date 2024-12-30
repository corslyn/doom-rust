use crate::{Map, Wad};

impl Map {
    pub fn new(wad: &Wad, map_name: &str) -> Map {
        let vertexes = wad.get_vertices(map_name);
        let linedefs = wad.get_linedefs(map_name);
        let subsectors = wad.get_subsectors(map_name);
        let things = wad.get_things(map_name);
        let segments = wad.get_segments(map_name);
        let nodes = wad.get_nodes(map_name);
        Map {
            name: map_name.to_string(),
            vertexes,
            linedefs,
            subsectors,
            things,
            segments,
            nodes,
        }
    }
}
