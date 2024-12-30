use crate::{Player, Thing};

impl Player {
    pub fn new(things: &Vec<Thing>) -> Player {
        let thing = &things[0];
        let angle = thing.angle;
        let pos = (thing.x, thing.y);
        Player {
            thing: thing.thing_type,
            angle,
            pos,
        }
    }
}
