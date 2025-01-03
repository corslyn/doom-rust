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
            subsectors: wad.get_subsectors(map_name),
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
        //self.render_automap_node(canvas);
        self.render_bsp_nodes(canvas, self.nodes.len() - 1);
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

    fn render_automap_node(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        node_id: usize,
    ) {
        let node = &self.nodes[node_id]; // root node

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
        let padding = 30;
        let padded = 320 * 4 - 2 * padding;
        let map_width = (self.x_max - self.x_min) as f32;
        let scale_x = padded as f32 / map_width;
        let offset_x = padding as f32;
        (((x - self.x_min) as f32) * scale_x + offset_x) as i32
    }

    fn remap_y_to_screen(&self, y: i16) -> i32 {
        let padding = 30;
        let padded = 200 * 4 - 2 * padding;

        let map_height = (self.y_max - self.y_min) as f32;
        let scale_y = padded as f32 / map_height;
        let offset_y = padding as f32;

        let mapped_y = (y - self.y_min) as f32 * scale_y + offset_y;

        let flipped_y = (200 * 4) as f32 - mapped_y;

        flipped_y.round() as i32
    }

    fn is_on_left_side(&self, x: i16, y: i16, node_id: usize) -> bool {
        let node = &self.nodes[node_id as usize];
        let dx = x as i32 - node.x_partition as i32;
        let dy = y as i32 - node.y_partition as i32;

        ((dx * node.change_y_partition as i32) - (dy * node.change_x_partition as i32)) < 0
    }

    fn render_bsp_nodes(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        node_id: usize,
    ) {
        let subsector_identifier = 0x8000;

        if (node_id & subsector_identifier) != 0 {
            self.render_subsector();
            return;
        }

        let is_on_left = self.is_on_left_side(
            self.player.x_position as i16,
            self.player.y_position as i16,
            node_id,
        );

        if is_on_left {
            self.render_bsp_nodes(canvas, self.nodes[node_id].left_child as usize);
            self.render_bsp_nodes(canvas, self.nodes[node_id].right_child as usize);
        } else {
            self.render_bsp_nodes(canvas, self.nodes[node_id].right_child as usize);
            self.render_bsp_nodes(canvas, self.nodes[node_id].left_child as usize);
        }
    }

    fn render_subsector(&self) {}
}
