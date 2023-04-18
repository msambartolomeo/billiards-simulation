use super::constants::Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    Ball(usize),
    Wall(Wall),
    Hole,
}

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub time: f32,
    pub ball: usize,
    pub collision_type: CollisionType,
}

impl Event {
    pub fn new(time: f32, ball: usize, collision_type: CollisionType) -> Self {
        Self {
            time,
            ball,
            collision_type,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        // NOTE: We assume a ball can't collide with two objects at the exact same time
        self.time == other.time
            && self.ball == other.ball
            && self.collision_type == other.collision_type
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.total_cmp(&other.time)
    }
}
