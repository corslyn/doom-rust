use crate::Vertex;

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
