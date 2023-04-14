use core::panic;

use super::constants::{
    Hole, Wall, BALL_MASS, BALL_RADIUS, TABLE_LENGTH, TABLE_WIDTH, WALL_VARIANTS,
};

pub struct Ball {
    x: f64,
    y: f64,
    v_x: f64,
    v_y: f64,
    r: f64,
    mass: u32,
}

impl Ball {
    pub fn new(x: f64, y: f64) -> Self {
        Ball {
            x,
            y,
            v_x: 0.0,
            v_y: 0.0,
            r: BALL_RADIUS,
            mass: BALL_MASS,
        }
    }
}

pub trait Collide {
    fn get_ball_collision_time(&self, other: &Ball) -> f64;
    fn get_wall_collision_time(&self) -> (Wall, f64);
    fn get_hole_collision_time(&self) -> Option<(Hole, f64)>;

    fn collide_ball(&mut self, other: &mut Ball);
    fn collide_wall(&mut self, other: &mut Wall);
}

impl Collide for Ball {
    fn get_ball_collision_time(&self, other: &Ball) -> f64 {
        let delta_coords = (self.x - other.x, self.y - other.y);
        let delta_vel = (self.v_x - other.v_x, self.v_y - other.v_y);

        let coords_dot_vel = delta_coords.0 * delta_vel.0 + delta_coords.1 * delta_vel.1;
        if coords_dot_vel >= 0.0 {
            return std::f64::INFINITY;
        }
        let sigma = self.r + other.r;
        let coords_dot_coords = delta_coords.0 * delta_coords.0 + delta_coords.1 * delta_coords.1;
        let vel_dot_vel = delta_vel.0 * delta_vel.0 + delta_vel.1 * delta_vel.1;
        let discriminant =
            coords_dot_vel * coords_dot_vel - vel_dot_vel * (coords_dot_coords - sigma * sigma);
        if discriminant < 0.0 {
            return std::f64::INFINITY;
        }
        (-coords_dot_vel - discriminant.sqrt()) / vel_dot_vel
    }

    fn get_wall_collision_time(&self) -> (Wall, f64) {
        let mut times: Vec<(Wall, f64)> = Vec::with_capacity(WALL_VARIANTS.len());

        for wall in WALL_VARIANTS {
            match wall {
                Wall::Top => {
                    if self.v_y > 0.0 {
                        let time = (TABLE_WIDTH - self.y - self.r) / self.v_y;
                        times.push((wall, time));
                    }
                }
                Wall::Bottom => {
                    if self.v_y < 0.0 {
                        let time = (self.y - self.r) / self.v_y;
                        times.push((wall, time));
                    }
                }
                Wall::Left => {
                    if self.v_x > 0.0 {
                        let time = (self.x - self.r) / self.v_x;
                        times.push((wall, time));
                    }
                }
                Wall::Right => {
                    if self.v_x < 0.0 {
                        let time = (TABLE_LENGTH - self.x - self.r) / self.v_x;
                        times.push((wall, time));
                    }
                }
            }
        }

        if times.is_empty() {
            panic!("No wall collision time found");
        }
        times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        times[0]
    }

    fn get_hole_collision_time(&self) -> Option<(Hole, f64)> {
        todo!()
    }

    fn collide_ball(&mut self, other: &mut Ball) {
        let delta_coords = (other.x - self.x, other.y - self.y);
        let delta_vel = (other.v_x - self.v_x, other.v_y - self.v_y);
        let vel_dot_coords = delta_vel.0 * delta_coords.0 + delta_vel.1 * delta_coords.1;
        let sigma = self.r + other.r;

        let j = 2.0 * self.mass as f64 * other.mass as f64 * vel_dot_coords
            / (sigma * (self.mass + other.mass) as f64);

        let j_x = j * delta_coords.0 / sigma;
        let j_y = j * delta_coords.1 / sigma;

        self.v_x += j_x / self.mass as f64;
        self.v_y += j_y / self.mass as f64;
        other.v_x -= j_x / other.mass as f64;
        other.v_y -= j_y / other.mass as f64;
    }

    fn collide_wall(&mut self, other: &mut Wall) {
        match other {
            Wall::Top | Wall::Bottom => {
                self.v_y = -self.v_y;
            }
            Wall::Right | Wall::Left => {
                self.v_x = -self.v_x;
            }
        }
    }
}
