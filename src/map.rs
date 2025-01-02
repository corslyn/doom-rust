use sdl2::pixels::Color;

use crate::data_types::{Map, Vertex, Wad};

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
        Map {
            map_name: map_name.to_string(),
            vertices: Vec::new(),
            linedefs: wad.get_linedefs(map_name),
            x_min: i16::MAX,
            x_max: i16::MIN,
            y_min: i16::MAX,
            y_max: i16::MIN,
        }
    }

    pub fn render_automap(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let render_x_size = canvas.output_size().unwrap().0 - 1;
        let render_y_size = canvas.output_size().unwrap().1 - 1;

        let x_shift = -self.x_min;
        let y_shift = -self.y_min;
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
}
