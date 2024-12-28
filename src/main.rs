#![allow(dead_code)]

use types::*;

mod config;
mod data;
mod render;
mod types;
mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    let map = Map::new(&wad, "E1M1");

    println!("{:?}", map.nodes);
    // render::render(map);
}
