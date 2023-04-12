use std::collections::BTreeSet;

use collision::Ball;
use constants::BALL_COUNT;
use event::Event;

mod collision;
mod constants;
mod event;

pub struct Table {
    balls: Vec<Ball>,
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
        let mut balls = Vec::with_capacity(BALL_COUNT);

        for i in 0..BALL_COUNT {
            // TODO: create real balls
            let ball = Ball::new(i, 0.0, 0.0);

            balls.push(ball);
        }

        let events = BTreeSet::new();
        // TODO: Calculate events

        Table { balls, events }
    }
}
