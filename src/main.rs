#![allow(dead_code)]

use data_types::{Map, Wad};
use engine::Engine;

mod data_types;
mod engine;
mod map;
mod wad;

fn main() {
    let wad = Wad::new("wad/DOOM.WAD");
    let map = Map::new(&wad, "E4M1");
    let mut engine = Engine::new(wad, map);

    while engine.running {
        // Main loop
        engine.render();
        engine.canvas.present();
    }
}
