use crate::data_types::Map;

impl Map {
    pub fn new(map_name: &str) -> Map {
        Map {
            map_name: map_name.to_string(),
            vertices: Vec::new(),
            linedefs: Vec::new(),
        }
    }
}
