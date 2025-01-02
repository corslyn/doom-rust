use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, rect::Rect};

use crate::{
    data_types::{Map, Wad},
    player::Player,
};

pub enum LumpIndex {
    THINGS = 1,
    LINEDEFS,
    SIDEDEFS,
    VERTEXES,
    SEAGS,
    SSECTORS,
    NODES,
    SECTORS,
    REJECT,
    BLOCKMAP,
}

impl Map {
    pub fn new(wad: &Wad, map_name: &str) -> Map {
        let things = wad.get_map_things(map_name);
        Map {
            map_name: map_name.to_string(),
            vertices: Vec::new(),
            linedefs: wad.get_linedefs(map_name),
            things: things.clone(),
            nodes: wad.get_nodes(map_name),
            x_min: i16::MAX,
            x_max: i16::MIN,
            y_min: i16::MAX,
            y_max: i16::MIN,
            player: Player::new(things),
            scale_factor: 15.0 / 4.0,
            render_y_size: 0,
        }
    }

    pub fn render_automap(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.render_automap_walls(canvas);
        self.render_automap_player(canvas);
        self.render_automap_node(canvas);
    }

    fn render_automap_walls(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.render_y_size = canvas.output_size().unwrap().1 - 1;

        canvas.set_draw_color(Color::WHITE);

        for linedef in &self.linedefs {
            let start_vertex = &self.vertices[linedef.start_vertex as usize];
            let end_vertex = &self.vertices[linedef.end_vertex as usize];

            canvas
                .draw_line(
                    (
                        self.remap_x_to_screen(start_vertex.x_position),
                        self.remap_y_to_screen(start_vertex.y_position),
                    ),
                    (
                        self.remap_x_to_screen(end_vertex.x_position),
                        self.remap_y_to_screen(end_vertex.y_position),
                    ),
                )
                .unwrap();
        }
    }

    fn render_automap_player(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RED);

        let player_x = self.things[0].x_position;
        let player_y = self.things[0].y_position;

        canvas
            .filled_circle(
                self.remap_x_to_screen(player_x) as i16,
                self.remap_y_to_screen(player_y) as i16,
                1,
                Color::RED,
            )
            .unwrap();
    }

    fn render_automap_node(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let node = self.nodes.last().unwrap(); // root node

        canvas.set_draw_color(Color::GREEN);
        canvas
            .draw_rect(Rect::new(
                self.remap_x_to_screen(node.right_bbox_left),
                self.remap_y_to_screen(node.right_bbox_top),
                (self.remap_x_to_screen(node.right_bbox_right)
                    - self.remap_x_to_screen(node.right_bbox_left)
                    + 1)
                .try_into()
                .unwrap(),
                (self.remap_y_to_screen(node.right_bbox_bottom)
                    - self.remap_y_to_screen(node.right_bbox_top)
                    + 1)
                .try_into()
                .unwrap(),
            ))
            .unwrap();

        canvas.set_draw_color(Color::RED);
        canvas
            .draw_rect(Rect::new(
                self.remap_x_to_screen(node.left_bbox_left),
                self.remap_y_to_screen(node.left_bbox_top),
                (self.remap_x_to_screen(node.left_bbox_right)
                    - self.remap_x_to_screen(node.left_bbox_left)
                    + 1)
                .try_into()
                .unwrap(),
                (self.remap_y_to_screen(node.left_bbox_bottom)
                    - self.remap_y_to_screen(node.left_bbox_top)
                    + 1)
                .try_into()
                .unwrap(),
            ))
            .unwrap();

        canvas.set_draw_color(Color::BLUE);
        canvas
            .draw_line(
                (
                    self.remap_x_to_screen(node.x_partition),
                    self.remap_y_to_screen(node.y_partition),
                ),
                (
                    self.remap_x_to_screen(node.x_partition + node.change_x_partition),
                    self.remap_y_to_screen(node.y_partition + node.change_y_partition),
                ),
            )
            .unwrap();
    }

    fn remap_x_to_screen(&self, x: i16) -> i32 {
        ((x + (-self.x_min)) as f32 / self.scale_factor) as i32
    }

    fn remap_y_to_screen(&self, y: i16) -> i32 {
        (self.render_y_size as i16 - ((y + (-self.y_min)) as f32 / self.scale_factor) as i16)
            .try_into()
            .unwrap()
    }
}
