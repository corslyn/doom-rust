#![allow(dead_code)]

use types::*;

mod bsp;
mod config;
mod data;
mod map;
mod player;
mod render;
mod types;
mod utils;
mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    let map = Map::new(&wad, "E1M1");

    //println!("{:?}", map.nodes);
    render::render(map);
}
