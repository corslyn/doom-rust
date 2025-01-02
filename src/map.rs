use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

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
            x_min: i16::MAX,
            x_max: i16::MIN,
            y_min: i16::MAX,
            y_max: i16::MIN,
            player: Player::new(things),
        }
    }

    pub fn render_automap(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let x_shift = -self.x_min;
        let y_shift = -self.y_min;

        self.render_automap_walls(canvas, x_shift, y_shift);
        self.render_automap_player(canvas, x_shift, y_shift);
    }

    fn render_automap_walls(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        x_shift: i16,
        y_shift: i16,
    ) {
        let render_y_size = canvas.output_size().unwrap().1 - 1;

        let scale_factor = 15;
        canvas.set_draw_color(Color::WHITE);

        for linedef in &self.linedefs {
            let start_vertex = &self.vertices[linedef.start_vertex as usize];
            let end_vertex = &self.vertices[linedef.end_vertex as usize];

            canvas
                .draw_line(
                    (
                        ((start_vertex.x_position + x_shift) / scale_factor) as i32,
                        ((render_y_size as i16 - (start_vertex.y_position + y_shift) / scale_factor)
                            as i32),
                    ),
                    (
                        ((end_vertex.x_position + x_shift) / scale_factor) as i32,
                        ((render_y_size as i16 - (end_vertex.y_position + y_shift) / scale_factor)
                            as i32),
                    ),
                )
                .unwrap();
        }
    }

    fn render_automap_player(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        x_shift: i16,
        y_shift: i16,
    ) {
        let render_y_size = canvas.output_size().unwrap().1 - 1;

        let scale_factor = 15;
        canvas.set_draw_color(Color::RED);

        let player_x = self.things[0].x_position;
        let player_y = self.things[0].y_position;

        canvas
            .filled_circle(
                (player_x + x_shift) / scale_factor,
                render_y_size as i16 - ((player_y + y_shift) / scale_factor),
                1,
                Color::RED,
            )
            .unwrap();
    }
}
