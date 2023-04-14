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

                    ball.collide_ball(other_ball);
                }
                16..=21 => {
                    let ball = &mut self.balls[current.ball];
                    // FIXME: remove mutability from consts
                    let hole = &mut HOLE_VARIANTS[current.collidable - BALL_COUNT];

                    // TODO: Delete ball
                    todo!()
                }
                22..=25 => {
                    let ball = &mut self.balls[current.ball];
                    let wall = &mut WALL_VARIANTS[current.collidable - BALL_COUNT];

                    ball.collide_wall(wall);
                }
                _ => panic!("boom"),
            }

            // NOTE: advance time of events
            self.events.iter_mut().for_each(|e| e.time -= current.time);

            // If colidable is ball calculate its new events
            if (0..=15).contains(&current.collidable) {
                self.calculate_new_ball_events(current.collidable);
            }

            self.calculate_new_ball_events(current.ball);

            return true;
        }

        false
    }

    fn calculate_new_ball_events(&mut self, ball_id: usize) {
        self.events
            .retain(|e| e.ball != ball_id && e.collidable != ball_id);

        let other_ball = &self.balls[ball_id];

        for (idx, ball) in self.balls.iter().enumerate() {
            if idx != ball_id {
                let time = other_ball.get_ball_collision_time(ball);
                let event = Event::new(time, ball_id, idx);

                self.events.push(event);
            }
        }

        // TODO: add holes and walls
    }
}
