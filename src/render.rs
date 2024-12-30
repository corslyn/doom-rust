// Renders the automap for now

extern crate sdl2;

use config::H_FOV;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use utils::*;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

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
        );
        draw_node(
            &map.nodes,
            map.nodes.len() - 1,
            &mut canvas,
            &remap_x,
            &remap_y,
        );*/
        /*
        bsp::render(
            &map.subsectors,
            &map.nodes,
            &map.segments,
            &map.vertexes,
            &player,
            &mut canvas,
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
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => player.angle += 5,

                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // ~60 FPS
    }
}

/// Renders the automap
fn automap(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    map: &Map,
    player: &Player,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    canvas.set_draw_color(Color::RED);
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
    /*
        // Draw vertices
        for vertex in &map.vertexes {
            // Remaps all coordinates so that they fit on the window
            let x = remap_x(vertex.x_position);
            let y = remap_y(vertex.y_position);

            canvas
                .filled_circle(x as i16, y as i16, 1, Color::RGB(255, 255, 255))
                .unwrap(); // Draw vertices as circles
        }
    */
    // Place things on the map
    // Only draws the player for now
    for thing in &map.things {
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
            _ => {}
        }
    }
    bsp::render(
        &map.subsectors,
        &map.nodes,
        &map.segments,
        &map.vertexes,
        player,
        canvas,
        remap_x,
        remap_y,
    );

    draw_fov(canvas, player, remap_x, remap_y);
}

fn draw_player_pos(pos: (i16, i16), canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let x = pos.0;
    let y = pos.1;
    canvas
        .filled_circle(x, y, 2, Color::RGB(0, 255, 0))
        .unwrap() // Player green
}

fn draw_node(
    nodes: &Vec<Node>,
    node_id: usize,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    let node = &nodes[node_id];
    let bbox_front = &node.r_box;
    let bbox_back = &node.l_box;
    draw_bbox(bbox_front, Color::GREEN, canvas, remap_x, remap_y);
    draw_bbox(bbox_back, Color::RED, canvas, remap_x, remap_y);
    canvas.set_draw_color(Color::BLUE);
    canvas
        .draw_line(
            (remap_x(node.x_start), remap_y(node.y_start)),
            (
                remap_x(node.x_start + node.dx_start),
                remap_y(node.y_start + node.dy_start),
            ),
        )
        .unwrap();
}

/// Draws a bounding box
fn draw_bbox(
    bbox: &BBox,
    color: Color,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    let x1 = remap_x(bbox.left);
    let x2 = remap_x(bbox.right);
    let y1 = remap_y(bbox.top);
    let y2 = remap_y(bbox.bottom);
    canvas.set_draw_color(color);
    canvas
        .draw_rect(sdl2::rect::Rect::new(
            x1 as i32,
            y1 as i32,
            (x2 - x1) as u32,
            (y2 - y1) as u32,
        ))
        .unwrap();
}

pub fn draw_segment(
    vertexes: &Vec<Vertex>,
    segment: &Segment,
    subsector_id: i16,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    let vertex1 = &vertexes[segment.start as usize];
    let vertex2 = &vertexes[segment.end as usize];
    canvas.set_draw_color(Color::GREEN);
    canvas
        .draw_line(
            (remap_x(vertex1.x_position), remap_y(vertex1.y_position)),
            (remap_x(vertex2.x_position), remap_y(vertex2.y_position)),
        )
        .unwrap();
    //canvas.present();
}
fn draw_fov(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    player: &Player,
    remap_x: &Box<dyn Fn(i16) -> i32>,
    remap_y: &Box<dyn Fn(i16) -> i32>,
) {
    let (x, y) = (remap_x(player.pos.0), remap_y(player.pos.1));

    // The player's angle in radians
    let player_angle = (player.angle as f64 + 90.0).to_radians();

    // Calculate FOV boundaries
    let left_angle = player_angle - H_FOV.to_radians(); // Left boundary of FOV
    let right_angle = player_angle + H_FOV.to_radians(); // Right boundary of FOV

    let len_ray = WIN_RES.1 as f64; // Length of the ray lines

    let (x1, y1) = (
        remap_x((player.pos.0 as f64 + len_ray * left_angle.sin()) as i16),
        remap_y((player.pos.1 as f64 + len_ray * left_angle.cos()) as i16),
    );
    let (x2, y2) = (
        remap_x((player.pos.0 as f64 + len_ray * right_angle.sin()) as i16),
        remap_y((player.pos.1 as f64 + len_ray * right_angle.cos()) as i16),
    );

    canvas.set_draw_color(Color::YELLOW);
    canvas.draw_line((x as i32, y as i32), (x1, y1)).unwrap();
    canvas.draw_line((x as i32, y as i32), (x2, y2)).unwrap();
}
