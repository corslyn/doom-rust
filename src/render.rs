extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use crate::config::WIN_RES;
use crate::data::{Map, Vertex};

pub fn render(map: Map) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIN_RES.0, WIN_RES.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Create remapping functions for x and y
    let remap_x = remap_x(&map.vertexes, WIN_RES.0);
    let remap_y = remap_y(&map.vertexes, WIN_RES.1);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
        canvas.clear();

        // Draw vertices
        canvas.set_draw_color(Color::RGB(255, 255, 255)); // White for vertices
        for vertex in &map.vertexes {
            let x = remap_x(vertex.x_position);
            let y = remap_y(vertex.y_position);
            canvas.draw_point((x, y)).unwrap();
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // ~60 FPS
    }
}

pub fn remap_x(vertices: &Vec<Vertex>, win_width: u32) -> Box<dyn Fn(i16) -> i32> {
    // Determine map bounds for x-axis
    let (min_x, max_x) = vertices
        .iter()
        .fold((i16::MAX, i16::MIN), |(min_x, max_x), vertex| {
            (min_x.min(vertex.x_position), max_x.max(vertex.x_position))
        });

    // Calculate scaling factor and offset for x-axis
    let map_width = (max_x - min_x) as f32;
    let scale_x = win_width as f32 / map_width;
    let offset_x = 30.0;

    // Return a closure to perform x-coordinate remapping
    Box::new(move |x: i16| -> i32 { (((x - min_x) as f32) * scale_x + offset_x) as i32 })
}

pub fn remap_y(vertices: &Vec<Vertex>, win_height: u32) -> Box<dyn Fn(i16) -> i32> {
    // Determine map bounds for y-axis
    let (min_y, max_y) = vertices
        .iter()
        .fold((i16::MAX, i16::MIN), |(min_y, max_y), vertex| {
            (min_y.min(vertex.y_position), max_y.max(vertex.y_position))
        });

    // Calculate scaling factor and offset for y-axis
    let map_height = (max_y - min_y) as f32;
    let scale_y = win_height as f32 / map_height;
    let offset_y = 30.0;

    // Return a closure to perform y-coordinate remapping
    Box::new(move |y: i16| -> i32 { (((y - min_y) as f32) * scale_y + offset_y) as i32 })
}
