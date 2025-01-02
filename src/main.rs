#![allow(dead_code)]

use data_types::{Map, Wad};
use engine::Engine;

mod data_types;
mod engine;
mod map;
mod player;
mod wad;

fn main() {
    let wad = Wad::new("wad/DOOM.WAD");
    let mut map = Map::new(&wad, "E1M1");
    map.vertices = wad.get_vertices(&mut map);
    let mut engine = Engine::new(wad, map);

    while engine.running {
        // Main loop
        engine.render();
        engine.canvas.present();
    }
}
