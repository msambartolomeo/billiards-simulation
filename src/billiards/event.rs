pub struct Event {
    pub time: f64,
    pub ball: usize,
    pub collidable: usize,
}

impl Event {
    fn new(time: f64, ball: usize, collidable: usize) -> Self {
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
        self.time == other.time && self.ball == other.ball && self.collidable == other.collidable
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}
