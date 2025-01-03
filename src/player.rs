use crate::{
    angle::Angle,
    data_types::{Thing, Vertex},
};

pub struct Player {
    pub x_position: f32,
    pub y_position: f32,
    pub angle: Angle,
    pub fov: f32,
    rotation_speed: u16,
}

impl Player {
    pub fn new(things: Vec<Thing>) -> Player {
        let player_thing = &things[0];
        let angle = Angle::new(player_thing.angle as f32);
        Player {
            x_position: player_thing.x_position as f32,
            y_position: player_thing.y_position as f32,
            angle,
            fov: 90.0,
            rotation_speed: 4,
        }
    }

    pub fn angle_to_vertex(&self, vertex: &Vertex) -> Angle {
        let vdx = vertex.x_position as f32 - self.x_position;
        let vdy = vertex.y_position as f32 - self.y_position;
        let angle = (vdy).atan2(vdx).to_degrees();
        Angle { angle }
    }

    pub fn clip_vertexes_in_fov(
        &self,
        vertex1: &Vertex,
        vertex2: &Vertex,
        _angle1: Angle,
        _angle2: Angle,
    ) -> bool {
        let mut angle1 = Angle::new(self.angle_to_vertex(vertex1).get_angle());
        let mut angle2 = Angle::new(self.angle_to_vertex(vertex2).get_angle());

        // Normalize angles relative to the player's current angle
        angle1.set_angle(angle1.get_angle() - self.angle.get_angle());
        angle2.set_angle(angle2.get_angle() - self.angle.get_angle());
        angle1.normalize_angle();
        angle2.normalize_angle();

        // Check if angles are within FOV
        angle1.get_angle() <= self.fov || angle2.get_angle() <= self.fov
    }

    pub fn rotate_left(&mut self) {
        self.angle
            .set_angle(self.angle.get_angle() + 0.1875 * self.rotation_speed as f32);
    }

    pub fn rotate_right(&mut self) {
        self.angle
            .set_angle(self.angle.get_angle() - 0.1875 * self.rotation_speed as f32);
    }
}
