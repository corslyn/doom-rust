#![allow(dead_code)]

use data_types::Wad;

mod data_types;
mod map;
mod wad;

fn main() {
    let wad = Wad::new("wad/DOOM.WAD");
    wad.read_directory();
}
