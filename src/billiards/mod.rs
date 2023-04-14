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
    balls: Vec<Option<Ball>>,
    events: Vec<Event>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new(true, 0.0, 1.0)
    }
}

impl Table {
    pub fn new(fixed_ball_spacing: bool, white_offset: f64, initial_velocity: f64) -> Self {
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

        let white_ball = Ball::with_velocity(
            TABLE_WIDTH / 2.0,
            TABLE_WIDTH / 2.0 + white_offset,
            initial_velocity,
            0.0,
        );

        balls.push(Some(white_ball));

        for (x, y) in constants::get_balls_starting_position() {
            let x_spacing = get_ball_spacing();
            let y_spacing = get_ball_spacing();
            let ball = Ball::new(x + x_spacing, y + y_spacing);

            balls.push(Some(ball));
        }

        assert_eq!(balls.len(), 16);

        let events = vec![];

        let mut pool = Self { balls, events };

        // NOTE: calculate events of white ball
        pool.calculate_new_ball_events(0);

        pool
    }

    pub fn handle_event(&mut self) -> bool {
        if let Some(current) = self.events.iter().min().cloned() {
            match current.collidable {
                0..=15 => {
                    let first;
                    let second;
                    if current.collidable < current.ball {
                        first = current.collidable;
                        second = current.ball;
                    } else {
                        first = current.ball;
                        second = current.collidable;
                    }

                    let (ball, other_ball) =
                        if let [first, .., second] = &mut self.balls[first..=second] {
                            (first.as_mut(), second.as_mut())
                        } else {
                            panic!("invalid event indexes");
                        };

                    if let (Some(ball), Some(other_ball)) = (ball, other_ball) {
                        ball.collide_ball(other_ball);
                    }
                }
                16..=21 => {
                    // NOTE: Remove ball from array
                    self.balls[current.ball].take();
                }
                22..=25 => {
                    if let Some(ball) = self.balls[current.ball].as_mut() {
                        let wall = &WALL_VARIANTS[current.collidable - BALL_COUNT];

                        ball.collide_wall(wall);
                    }
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

        if let Some(other_ball) = self.balls[ball_id].as_ref() {
            for (idx, ball) in self.balls.iter().flatten().enumerate() {
                if idx != ball_id {
                    let time = other_ball.get_ball_collision_time(ball);
                    let event = Event::new(time, ball_id, idx);

                    self.events.push(event);
                }
            }

            // TODO: add holes and walls
        }
    }
}
