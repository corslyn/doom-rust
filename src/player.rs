use crate::data_types::Thing;

pub struct Player {
    pub x_position: f32,
    pub y_position: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(things: Vec<Thing>) -> Player {
        let player_thing = &things[0];
        Player {
            x_position: player_thing.x_position as f32,
            y_position: player_thing.y_position as f32,
            angle: player_thing.angle as f32,
        }
    }
}
