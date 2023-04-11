use super::collision::{Ball, Collidable};

pub struct Event {
    time: f64,
    ball: Ball,
    collidable: Box<dyn Collidable>,
}

impl Event {
    fn new(time: f64, ball: Ball, collidable: Box<dyn Collidable>) -> Self {
        Self {
            time,
            ball,
            collidable,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        // NOTE: We assume a ball can't collide with two objects at the exact same time
        self.time == other.time && self.ball == other.ball
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}
