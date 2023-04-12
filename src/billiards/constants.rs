pub const TABLE_WIDTH: f64 = 112.0;
pub const TABLE_LENGTH: f64 = 224.0;
pub const HOLE_RADIUS: f64 = BALL_RADIUS * 2.0;
pub const BALL_COUNT: usize = 16;
pub const BALL_MASS: u32 = 165;
pub const BALL_RADIUS: f64 = 5.7 / 2.0;
pub const NOISE_LOWER_BOUND: f64 = 0.02;
pub const NOISE_UPPER_BOUND: f64 = 0.03;

pub const HOLE_VARIANTS: [Hole; 6] = [
    Hole::BottomLeft,
    Hole::BottomMiddle,
    Hole::BottomRight,
    Hole::TopLeft,
    Hole::TopMiddle,
    Hole::TopRight,
];

pub const WALL_VARIANTS: [Wall; 4] = [Wall::Top, Wall::Bottom, Wall::Left, Wall::Right];

pub enum Hole {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    TopLeft,
    TopMiddle,
    TopRight,
}

impl Hole {
    pub fn coordinates(&self) -> (f64, f64) {
        match self {
            Hole::BottomLeft => (0.0, 0.0),
            Hole::BottomMiddle => (TABLE_LENGTH / 2.0, 0.0),
            Hole::BottomRight => (TABLE_LENGTH, 0.0),
            Hole::TopLeft => (0.0, TABLE_WIDTH),
            Hole::TopMiddle => (TABLE_LENGTH / 2.0, TABLE_WIDTH),
            Hole::TopRight => (TABLE_LENGTH, TABLE_WIDTH),
        }
    }

    pub fn radius(&self) -> f64 {
        HOLE_RADIUS
    }
}

pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Wall {
    pub fn coordinate(&self) -> f64 {
        match self {
            Wall::Top => TABLE_WIDTH,
            Wall::Bottom | Wall::Left => 0.0,
            Wall::Right => TABLE_LENGTH,
        }
    }
}
