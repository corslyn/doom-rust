// Renders the automap for now

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use utils::*;

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

    let mut automap_active = true; // Flag for automap toggle
    let mut player = Player::new(&map.things);
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
        canvas.clear();

        if automap_active {
            // Draw automap if active
            automap(&mut canvas, &map, &player, &remap_x, &remap_y);
        }

        //draw_bbox(&mut canvas, &map.nodes, &player, &remap_x, &remap_y);

        /*render_subsectors(
            &mut canvas,
            &map.subsectors,
            &map.segments,
            &map.vertexes,
            &remap_x,
            &remap_y,
        );*/

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => automap_active = !automap_active, // Toggle automap state
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => player.pos.1 += 10,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => player.pos.1 -= 10,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => player.pos.0 -= 10,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => player.pos.0 += 10,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // ~60 FPS
    }
}

fn draw_bbox(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    nodes: &Vec<Node>,
    player: &Player,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    // left rectangle
    let node = bsp::find_player_node(
        player.pos.0,
        player.pos.1,
        &nodes,
        (nodes.len() - 1).try_into().unwrap(),
    );
    let x1 = remap_x(node.l_box.right);
    let x2 = remap_x(node.l_box.left);
    let y1 = remap_y(node.l_box.bottom);
    let y2 = remap_y(node.l_box.top);
    canvas
        .rectangle(
            x1 as i16,
            y1 as i16,
            x2 as i16,
            y2 as i16,
            Color::RGB(255, 0, 0),
        )
        .unwrap();

    // right rectangle
    let x1 = remap_x(node.r_box.right);
    let x2 = remap_x(node.r_box.left);
    let y1 = remap_y(node.r_box.bottom);
    let y2 = remap_y(node.r_box.top);
    canvas
        .rectangle(
            x1 as i16,
            y1 as i16,
            x2 as i16,
            y2 as i16,
            Color::RGB(0, 255, 0),
        )
        .unwrap();
}

fn automap(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    map: &Map,
    player: &Player,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    canvas.set_draw_color(Color::RGB(70, 70, 70)); // gray
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
            1 => {
                draw_player_pos(
                    (
                        remap_x(player.pos.0).try_into().unwrap(),
                        remap_y(player.pos.1).try_into().unwrap(),
                    ),
                    canvas,
                );
            }
            _ => canvas
                .filled_circle(x as i16, y as i16, 2, Color::RGB(255, 128, 0))
                .unwrap(), // The rest is orange
        }
    }
}

fn draw_player_pos(pos: (i16, i16), canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let x = pos.0;
    let y = pos.1;
    canvas
        .filled_circle(x, y, 2, Color::RGB(0, 255, 0))
        .unwrap() // Player green
}
