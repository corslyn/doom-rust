#![allow(dead_code)]

use data::Map;
use wad::Wad;

mod config;
mod data;
mod render;
mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    let map = Map::new(&wad, "E1M9");
    let header = wad.read_header();
    println!("Wad size : {} bytes", wad.data.len());
    println!("Wad type : {}", header.wad_type);
    println!("Numlumps : {}", header.numlumps);
    println!("Info table offset : {}", header.infotableofs);
    println!("E1M1 index: {}", wad.get_lump_index(&map.name));
    // println!("Lumps : {:?}", wad.read_directory().lumps)
    // println!("Vertices: {:?}", wad.get_vertices(&map.name));
    println!("{:?}", map.linedefs);
    render::render(map);
}
