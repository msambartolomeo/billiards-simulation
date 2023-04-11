use super::constants::{Hole, Wall, BALL_MASS, BALL_RADIUS};

pub trait Collide<T> {
    fn collide(&mut self, other: &mut T);
    fn get_collision_time(&self, other: &T) -> f64;
}

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

// NOTE: Marker trait for all objects that can collide
pub trait Collidable {}

impl Collidable for Ball {}
impl Collidable for Wall {}
impl Collidable for Hole {}

impl Collide<Ball> for Ball {
    fn collide(&mut self, other: &mut Ball) {
        todo!()
    }

    fn get_collision_time(&self, other: &Ball) -> f64 {
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
}

impl Collide<Hole> for Ball {
    fn collide(&mut self, other: &mut Hole) {
        todo!()
    }

    fn get_collision_time(&self, other: &Hole) -> f64 {
        todo!()
    }
}

impl Collide<Wall> for Ball {
    fn collide(&mut self, other: &mut Wall) {
        todo!()
    }

    fn get_collision_time(&self, other: &Wall) -> f64 {
        todo!()
    }
}