#![allow(dead_code)]

use data_types::{Map, Wad};

mod data_types;
mod map;
mod wad;

fn main() {
    let wad = Wad::new("wad/DOOM.WAD");
    let map = Map::new(&wad, "E4M1");
    println!(
        "Map index of {} : {}",
        &map.map_name,
        wad.get_lump_index(&map.map_name)
    );

    for vertex in map.vertices {
        println!("({}, {})", vertex.x_position, vertex.y_position);
    }
}
