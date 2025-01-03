pub struct Angle {
    pub angle: f32,
}

impl Angle {
    pub fn new(angle: f32) -> Angle {
        Angle { angle }
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn normalize_angle(&mut self) {
        self.angle = self.angle % 360.0;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }
}
