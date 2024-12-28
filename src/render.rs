// Renders the automap for now

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use crate::config::WIN_RES;
use crate::*;

pub fn render(map: Map) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(&format!("DOOM-RUST: {}", map.name), WIN_RES.0, WIN_RES.1)
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

        // Draw linedefs
        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red for linedefs
        for linedef in &map.linedefs {
            let start = &map.vertexes[linedef.start as usize];
            let end = &map.vertexes[linedef.end as usize];
            // Remaps all coordinates so that they fit on the window
            let x1 = remap_x(start.x_position);
            let x2 = remap_x(end.x_position);
            let y1 = remap_y(start.y_position);
            let y2 = remap_y(end.y_position);
            canvas.draw_line((x1, y1), (x2, y2)).unwrap();
        }

        // Draw vertices
        for vertex in &map.vertexes {
            // Remaps all coordinates so that they fit on the window
            let x = remap_x(vertex.x_position);
            let y = remap_y(vertex.y_position);

            canvas
                .filled_circle(x as i16, y as i16, 1, Color::RGB(255, 255, 255))
                .unwrap(); // Draw vertices as circles
        }

        // Place things on the map
        for thing in &map.things {
            let x = remap_x(thing.x);
            let y = remap_y(thing.y);
            match thing.thing_type {
                1 => canvas
                    .filled_circle(x as i16, y as i16, 2, Color::RGB(0, 255, 0))
                    .unwrap(), // Player green
                _ => canvas
                    .filled_circle(x as i16, y as i16, 2, Color::RGB(255, 128, 0))
                    .unwrap(), // The rest is orange
            }
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
    let padding = 30.0;
    // Determine map bounds for x-axis
    let (min_x, max_x) = vertices
        .iter()
        .fold((i16::MAX, i16::MIN), |(min_x, max_x), vertex| {
            (min_x.min(vertex.x_position), max_x.max(vertex.x_position))
        });

    // Adjust available screen width with padding
    let padded_width = win_width as f32 - 2.0 * padding as f32;

    // Calculate scaling factor and offset for x-axis
    let map_width = (max_x - min_x) as f32;
    let scale_x = padded_width / map_width;
    let offset_x = padding as f32;

    // Return a closure to perform x-coordinate remapping
    Box::new(move |x: i16| -> i32 { (((x - min_x) as f32) * scale_x + offset_x) as i32 })
}
pub fn remap_y(vertices: &Vec<Vertex>, win_height: u32) -> Box<dyn Fn(i16) -> i32> {
    let padding = 30;

    // Determine map bounds for y-axis
    let (min_y, max_y) = vertices
        .iter()
        .fold((i16::MAX, i16::MIN), |(min_y, max_y), vertex| {
            (min_y.min(vertex.y_position), max_y.max(vertex.y_position))
        });

    // Adjust available screen height with padding
    let padded_height = win_height as f32 - 2.0 * padding as f32;

    // Calculate scaling factor and offset for y-axis
    let map_height = (max_y - min_y) as f32;
    let scale_y = padded_height / map_height;
    let offset_y = padding as f32;

    // Return a closure to perform y-coordinate remapping
    Box::new(move |y: i16| -> i32 {
        // Map the y-coordinate to screen space without flipping
        let mapped_y = (y - min_y) as f32 * scale_y + offset_y;

        // Flip the y-coordinate relative to the screen height
        let flipped_y = win_height as f32 - mapped_y;

        flipped_y.round() as i32
    })
}
