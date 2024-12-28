#![allow(dead_code)]

use data::Map;
use wad::Wad;

mod config;
mod data;
mod render;
mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    let map = Map::new(&wad, "E1M1");

    render::render(map);
}
