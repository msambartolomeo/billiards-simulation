use std::collections::HashMap;

use collision::{Ball, Collidable};
use constants::{Hole, Wall, BALL_COUNT};

mod collision;
mod constants;

pub struct Table {
    collidables: HashMap<usize, Box<dyn Collidable>>,
}

impl Table {
    pub fn new(dist: f64) -> Self {
        let mut collidables: HashMap<usize, Box<dyn Collidable>> = HashMap::new();

        for i in 0..BALL_COUNT {
            // TODO: create real balls
            let ball = Ball::new(0.0, 0.0);

            collidables.insert(i, Box::new(ball));
        }

        let mut i = BALL_COUNT;
        for hole in Hole::variants() {
            collidables.insert(i, Box::new(hole));
            i += 1;
        }
        for wall in Wall::variants() {
            collidables.insert(i, Box::new(wall));
            i += 1;
        }

        Table { collidables }
    }
}
