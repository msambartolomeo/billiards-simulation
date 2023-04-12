use std::ops::RangeInclusive;

pub const TABLE_WIDTH: f64 = 112.0;
pub const TABLE_LENGTH: f64 = 224.0;
pub const HOLE_RADIUS: f64 = BALL_RADIUS * 2.0;
pub const BALL_COUNT: usize = 16;
pub const BALL_MASS: u32 = 165;
pub const BALL_RADIUS: f64 = 5.7 / 2.0;
pub const BALL_SPACING_LOWER_BOUND: f64 = 0.02;
pub const BALL_SPACING_UPPER_BOUND: f64 = 0.03;
pub const BALL_SPACING_RANGE: RangeInclusive<f64> =
    BALL_SPACING_LOWER_BOUND..=BALL_SPACING_UPPER_BOUND;

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

const BALL_RADIUS_WITH_SPACING: f64 = BALL_RADIUS + BALL_SPACING_UPPER_BOUND;

pub fn get_balls_starting_position() -> Vec<(f64, f64)> {
    let mut positions = Vec::with_capacity(BALL_COUNT - 1);

    for row in 0..5 {
        let x = 5f64.sqrt() * BALL_RADIUS_WITH_SPACING * row as f64;

        for y in &Y_COORDINATES_PER_ROW[row] {
            if let Some(y) = y {
                positions.push((TABLE_LENGTH - TABLE_WIDTH / 2.0 + x, TABLE_WIDTH / 2.0 + y))
            }
        }
    }

    positions
}

const Y_COORDINATES_PER_ROW: [[Option<f64>; 5]; 5] = [
    [Some(0.0), None, None, None, None],
    [
        Some(-BALL_RADIUS_WITH_SPACING),
        Some(BALL_RADIUS_WITH_SPACING),
        None,
        None,
        None,
    ],
    [
        Some(-2.0 * BALL_RADIUS_WITH_SPACING),
        Some(0.0),
        Some(2.0 * BALL_RADIUS_WITH_SPACING),
        None,
        None,
    ],
    [
        Some(-3.0 * BALL_RADIUS_WITH_SPACING),
        Some(-BALL_RADIUS_WITH_SPACING),
        Some(BALL_RADIUS_WITH_SPACING),
        Some(3.0 * BALL_RADIUS_WITH_SPACING),
        None,
    ],
    [
        Some(-4.0 * BALL_RADIUS_WITH_SPACING),
        Some(-2.0 * BALL_RADIUS_WITH_SPACING),
        Some(0.0),
        Some(2.0 * BALL_RADIUS_WITH_SPACING),
        Some(4.0 * BALL_RADIUS_WITH_SPACING),
    ],
];
