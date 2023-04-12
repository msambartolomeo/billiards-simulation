use super::constants::{Hole, Wall, BALL_MASS, BALL_RADIUS, TABLE_LENGTH, TABLE_WIDTH};

pub trait Collide<T> {
    fn collide(&mut self, other: &mut T) -> bool;
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

impl Collide<Ball> for Ball {
    fn collide(&mut self, other: &mut Ball) -> bool {
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
    fn collide(&mut self, other: &mut Hole) -> bool {
        todo!()
    }

    fn get_collision_time(&self, other: &Hole) -> f64 {
        todo!()
    }
}

impl Collide<Wall> for Ball {
    fn collide(&mut self, other: &mut Wall) -> bool {
        match other {
            Wall::Top | Wall::Bottom => {
                self.v_y = -self.v_y;
            }
            Wall::Right | Wall::Left => {
                self.v_x = -self.v_x;
            }
        }
        false
    }

    fn get_collision_time(&self, other: &Wall) -> f64 {
        match other {
            Wall::Top => {
                if self.v_y <= 0.0 {
                    return std::f64::INFINITY;
                }
                (TABLE_WIDTH - self.y - self.r) / self.v_y
            }
            Wall::Right => {
                if self.v_x <= 0.0 {
                    return std::f64::INFINITY;
                }
                (TABLE_LENGTH - self.x - self.r) / self.v_x
            }
            Wall::Bottom => {
                if self.v_y >= 0.0 {
                    return std::f64::INFINITY;
                }
                (self.y - self.r) / self.v_y
            }
            Wall::Left => {
                if self.v_x >= 0.0 {
                    return std::f64::INFINITY;
                }
                (self.x - self.r) / self.v_x
            }
        }
    }
}
