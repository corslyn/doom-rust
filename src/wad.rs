use crate::data_types::Wad;

impl Wad {
    /// Creates a new Wad instance from a file
    pub fn new(filepath: &str) -> Wad {
        let wad_data = std::fs::read(filepath).expect("File not found");
        Wad { data: wad_data }
    }
}
