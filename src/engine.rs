use crate::data_types::{Map, Wad};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub struct Engine {
    pub map: Map,
    pub wad: Wad,
    pub resolution: (u32, u32),
    pub is_over: bool,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub sdl_context: sdl2::Sdl,
    pub title: String,
}

impl Engine {
    pub fn new(wad: Wad, map: Map) -> Engine {
        let resolution = (320, 200);
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let title = format!("Doom - {}", map.map_name);
        let window = video_subsystem
            .window(&title, resolution.0, resolution.1)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Engine {
            map,
            wad,
            resolution,
            is_over: false,
            canvas,
            sdl_context,
            title,
        }
    }

    /// Renders the game
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.present();
        self.process_input();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60 fps
    }

    /// Process the input events
    pub fn process_input(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_over = true,
                _ => {}
            }
        }
    }
}
