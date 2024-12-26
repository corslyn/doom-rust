use std::fs;

#[derive(Debug)]

pub struct Wad {
    pub data: Vec<u8>,
}

impl Wad {
    pub fn new(file_path: std::path::PathBuf) -> Wad {
        let data = fs::read(file_path).expect("Unable to read file");
        Wad { data }
    }
}
