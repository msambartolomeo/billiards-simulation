use super::constants::{
    Wall, BALL_MASS, BALL_RADIUS, HOLE_RADIUS, HOLE_VARIANTS, TABLE_LENGTH, TABLE_WIDTH,
    WALL_VARIANTS,
};

pub struct Ball {
    x: f64,
    y: f64,
    v_x: f64,
    v_y: f64,
    r: f64,
    mass: f64,
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

    pub fn with_velocity(x: f64, y: f64, v_x: f64, v_y: f64) -> Self {
        let mut ball = Self::new(x, y);
        ball.v_x = v_x;
        ball.v_y = v_y;
        ball
    }

    pub fn advance(&mut self, time: f64) {
        self.x += self.v_x * time;
        self.y += self.v_y * time;
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_velocity_x(&self) -> f64 {
        self.v_x
    }

    pub fn get_velocity_y(&self) -> f64 {
        self.v_y
    }

    pub fn get_radius(&self) -> f64 {
        self.r
    }
}

pub trait Collide {
    fn get_ball_collision_time(&self, other: &Ball) -> f64;
    fn get_wall_collision_time(&self) -> (Wall, f64);
    fn get_hole_collision_time(&self) -> Option<f64>;

    fn collide_ball(&mut self, other: &mut Ball);
    fn collide_wall(&mut self, other: Wall);
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
        let mut times = Vec::with_capacity(WALL_VARIANTS.len());

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
        times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        times[0]
    }

    fn get_hole_collision_time(&self) -> Option<f64> {
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
