const TABLE_WITDH: f64 = 112.0;
const TABLE_LENGTH: f64 = 224.0;
pub const BALL_COUNT: usize = 16;
pub const BALL_MASS: u32 = 165;
pub const BALL_RADIUS: f64 = 5.7 / 2.0;
const HOLE_RADIUS: f64 = BALL_RADIUS * 2.0;

pub enum Hole {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    TopLeft,
    TopMiddle,
    TopRight,
}

impl Hole {
    pub fn variants() -> [Hole; 6] {
        [
            Hole::BottomLeft,
            Hole::BottomMiddle,
            Hole::BottomRight,
            Hole::TopLeft,
            Hole::TopMiddle,
            Hole::TopRight,
        ]
    }

    pub fn coordinates(&self) -> (f64, f64) {
        match self {
            Hole::BottomLeft => (0.0, 0.0),
            Hole::BottomMiddle => (TABLE_LENGTH / 2.0, 0.0),
            Hole::BottomRight => (TABLE_LENGTH, 0.0),
            Hole::TopLeft => (0.0, TABLE_WITDH),
            Hole::TopMiddle => (TABLE_LENGTH / 2.0, TABLE_WITDH),
            Hole::TopRight => (TABLE_LENGTH, TABLE_WITDH),
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
    pub fn variants() -> [Wall; 4] {
        [Wall::Top, Wall::Bottom, Wall::Left, Wall::Right]
    }

    pub fn coordinate(&self) -> f64 {
        match self {
            Wall::Top => TABLE_WITDH,
            Wall::Bottom | Wall::Left => 0.0,
            Wall::Right => TABLE_LENGTH,
        }
    }
}
