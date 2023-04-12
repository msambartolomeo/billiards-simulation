use std::collections::BTreeSet;

use collision::Ball;
use constants::{BALL_COUNT, TABLE_WIDTH};
use event::Event;

use rand::Rng;

use self::constants::BALL_SPACING_RANGE;

mod collision;
mod constants;
mod event;

pub struct Table {
    balls: Vec<Ball>,
    events: BTreeSet<Event>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new(true, 0.0)
    }
}

impl Table {
    pub fn new(fixed_ball_spacing: bool, white_offset: f64) -> Self {
        let mut rng = rand::thread_rng();

        let mut get_ball_spacing = move || {
            if fixed_ball_spacing {
                0.0
            } else {
                let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                rng.gen_range(BALL_SPACING_RANGE) * sign
            }
        };

        let mut balls = Vec::with_capacity(BALL_COUNT);

        let white_ball = Ball::new(TABLE_WIDTH / 2.0, TABLE_WIDTH / 2.0 + white_offset);

        balls.push(white_ball);

        for (x, y) in constants::get_balls_starting_position() {
            let x_spacing = get_ball_spacing();
            let y_spacing = get_ball_spacing();
            // TODO: create real balls
            let ball = Ball::new(x + x_spacing, y + y_spacing);

            balls.push(ball);
        }

        assert_eq!(balls.len(), 16);

        let events = BTreeSet::new();
        // TODO: Calculate events

        Table { balls, events }
    }
}
