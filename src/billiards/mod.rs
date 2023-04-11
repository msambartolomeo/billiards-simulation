use std::collections::{BTreeSet, HashMap};

use collision::{Ball, Collidable};
use constants::{Hole, Wall, BALL_COUNT};

use self::event::Event;

mod collision;
mod constants;
mod event;

pub struct Table {
    collidables: HashMap<usize, Ball>,
    events: BTreeSet<Event>,
}

impl Default for Table {
    fn default() -> Self {
        // TODO: Calculate random dist
        let dist = 0.0;

        Self::new(dist)
    }
}

impl Table {
    pub fn new(dist: f64) -> Self {
        let mut collidables: HashMap<usize, Box<dyn Collidable>> = HashMap::new();

        for i in 0..BALL_COUNT {
            // TODO: create real balls
            let ball = Ball::new(i, 0.0, 0.0);

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

        let events = BTreeSet::new();
        // TODO: Calculate events

        Table {
            collidables,
            events,
        }
    }
}
