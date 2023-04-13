use collision::Ball;
use constants::{BALL_COUNT, TABLE_WIDTH};
use event::Event;

use rand::Rng;

use self::{
    collision::Collide,
    constants::{BALL_SPACING_RANGE, HOLE_VARIANTS, WALL_VARIANTS},
};

mod collision;
mod constants;
mod event;

pub struct Table {
    balls: Vec<Ball>,
    events: Vec<Event>,
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
            let ball = Ball::new(x + x_spacing, y + y_spacing);

            balls.push(ball);
        }

        assert_eq!(balls.len(), 16);

        // NOTE: Events is a normal vector but we handle the ordering
        let events = vec![];
        // TODO: Calculate events

        Table { balls, events }
    }

    pub fn handle_event(&mut self) -> bool {
        if let Some(current) = self.events.iter().min().cloned() {
            match current.collidable {
                0..=15 => {
                    let (ball, other_ball) = if current.collidable < current.ball {
                        let (a, b) = self.balls.split_at_mut(current.ball);
                        (&mut b[0], &mut a[current.collidable])
                    } else {
                        let (a, b) = self.balls.split_at_mut(current.collidable);
                        (&mut a[current.ball], &mut b[0])
                    };

                    ball.collide(other_ball);

                    self.events.retain(|e| {
                        e.ball != current.collidable && e.collidable != current.collidable
                    });

                    let mut i = 0;

                    // TODO: new events for other_ball
                }
                16..=21 => {
                    let ball = &mut self.balls[current.ball];
                    // FIXME: remove mutability from consts
                    let hole = &mut HOLE_VARIANTS[current.collidable - BALL_COUNT];

                    ball.collide(hole);
                }
                22..=25 => {
                    let ball = &mut self.balls[current.ball];
                    let wall = &mut WALL_VARIANTS[current.collidable - BALL_COUNT];

                    ball.collide(wall);
                }
                _ => panic!("boom"),
            }

            self.events
                .retain(|e| e.ball != current.ball && e.collidable != current.ball);

            // TODO: add events for ball

            // NOTE: advance time of events
            self.events
                .iter_mut()
                // NOTE: do not change the time of the newly added balls
                .filter(|e| e.ball != current.collidable && e.ball != current.ball)
                .for_each(|e| e.time -= current.time);

            return true;
        }

        false
    }
}
