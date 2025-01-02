#![allow(dead_code)]

use data_types::Wad;

mod data_types;
mod wad;

fn main() {
    let wad = Wad::new("wad/DOOM1.WAD");
}
