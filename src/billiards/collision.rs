use super::constants::{
    Wall, BALL_MASS, BALL_RADIUS, HOLE_RADIUS, HOLE_VARIANTS, TABLE_LENGTH, TABLE_WIDTH,
    WALL_VARIANTS,
};

#[derive(Debug, PartialEq)]
pub struct Ball {
    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,
    r: f32,
    mass: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Ball {
            x,
            y,
            v_x: 0.0,
            v_y: 0.0,
            r: BALL_RADIUS,
            mass: BALL_MASS,
        }
    }

    pub fn with_velocity(x: f32, y: f32, v_x: f32, v_y: f32) -> Self {
        let mut ball = Self::new(x, y);
        ball.v_x = v_x;
        ball.v_y = v_y;
        ball
    }

    pub fn advance(&mut self, time: f32) {
        self.x += self.v_x * time;
        self.y += self.v_y * time;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_velocity_x(&self) -> f32 {
        self.v_x
    }

    pub fn get_velocity_y(&self) -> f32 {
        self.v_y
    }

    pub fn get_radius(&self) -> f32 {
        self.r
    }

    pub fn get_distance(&self, other: &Ball) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub trait Collide {
    fn get_ball_collision_time(&self, other: &Ball) -> f32;
    fn get_wall_collision_time(&self) -> (Wall, f32);
    fn get_hole_collision_time(&self) -> Option<f32>;

    fn collide_ball(&mut self, other: &mut Ball);
    fn collide_wall(&mut self, other: Wall);
}

impl Collide for Ball {
    fn get_ball_collision_time(&self, other: &Ball) -> f32 {
        let delta_coords = (self.x - other.x, self.y - other.y);
        let delta_vel = (self.v_x - other.v_x, self.v_y - other.v_y);

        let coords_dot_vel = delta_coords.0 * delta_vel.0 + delta_coords.1 * delta_vel.1;
        if coords_dot_vel >= 0.0 {
            return f32::INFINITY;
        }
        let sigma = self.r + other.r;
        let coords_dot_coords = delta_coords.0 * delta_coords.0 + delta_coords.1 * delta_coords.1;
        let vel_dot_vel = delta_vel.0 * delta_vel.0 + delta_vel.1 * delta_vel.1;
        let discriminant =
            coords_dot_vel * coords_dot_vel - vel_dot_vel * (coords_dot_coords - sigma * sigma);
        if discriminant < 0.0 {
            return f32::INFINITY;
        }
        (-coords_dot_vel - discriminant.sqrt()) / vel_dot_vel
    }

    fn get_wall_collision_time(&self) -> (Wall, f32) {
        let mut times = Vec::new();
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
                        let time = (self.r - self.y) / self.v_y;
                        times.push((wall, time));
                    }
                }
                Wall::Left => {
                    if self.v_x < 0.0 {
                        let time = (self.r - self.x) / self.v_x;
                        times.push((wall, time));
                    }
                }
                Wall::Right => {
                    if self.v_x > 0.0 {
                        let time = (TABLE_LENGTH - self.x - self.r) / self.v_x;
                        times.push((wall, time));
                    }
                }
            }
        }

        assert!(!times.is_empty(), "No wall collision time found");
        times.sort_by(|a, b| a.1.total_cmp(&b.1));
        times[0]
    }

    fn get_hole_collision_time(&self) -> Option<f32> {
        for hole in &HOLE_VARIANTS {
            let (hole_x, hole_y) = hole.coordinates();
            let delta_coords = (self.x - hole_x, self.y - hole_y);
            let delta_vel = (self.v_x, self.v_y);

            let coords_dot_vel = delta_coords.0 * delta_vel.0 + delta_coords.1 * delta_vel.1;
            if coords_dot_vel >= 0.0 {
                continue;
            }
            let sigma = self.r + HOLE_RADIUS;
            let coords_dot_coords =
                delta_coords.0 * delta_coords.0 + delta_coords.1 * delta_coords.1;
            let vel_dot_vel = delta_vel.0 * delta_vel.0 + delta_vel.1 * delta_vel.1;
            let discriminant =
                coords_dot_vel * coords_dot_vel - vel_dot_vel * (coords_dot_coords - sigma * sigma);
            if discriminant < 0.0 {
                continue;
            }
            return Some((-coords_dot_vel - discriminant.sqrt()) / vel_dot_vel);
        }
        None
    }

    fn collide_ball(&mut self, other: &mut Ball) {
        let delta_coords = (other.x - self.x, other.y - self.y);
        let delta_vel = (other.v_x - self.v_x, other.v_y - self.v_y);
        let vel_dot_coords = delta_vel.0 * delta_coords.0 + delta_vel.1 * delta_coords.1;
        let sigma = self.r + other.r;

        let j = 2.0 * self.mass * other.mass * vel_dot_coords / (sigma * (self.mass + other.mass));

        let j_x = j * delta_coords.0 / sigma;
        let j_y = j * delta_coords.1 / sigma;

        self.v_x += j_x / self.mass;
        self.v_y += j_y / self.mass;
        other.v_x -= j_x / other.mass;
        other.v_y -= j_y / other.mass;
    }

    fn collide_wall(&mut self, other: Wall) {
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
