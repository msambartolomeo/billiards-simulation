pub trait Collide<T> {
    fn colide(&mut self, other: &mut T);
    fn get_colision_time(&self, other: &T) -> f64;
}

pub struct Ball {
    x: f64,
    y: f64,
    v_x: f64,
    v_y: f64,
    r: f64,
}

pub struct Hole {
    x: f64,
    y: f64,
    r: f64,
}

pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Collide<Ball> for Ball {
    fn colide(&mut self, other: &mut Ball) {
        todo!()
    }

    fn get_colision_time(&self, other: &Ball) -> f64 {
        todo!()
    }
}

impl Collide<Hole> for Ball {
    fn colide(&mut self, other: &mut Hole) {
        todo!()
    }

    fn get_colision_time(&self, other: &Hole) -> f64 {
        todo!()
    }
}

impl Collide<Wall> for Ball {
    fn colide(&mut self, other: &mut Wall) {
        todo!()
    }

    fn get_colision_time(&self, other: &Wall) -> f64 {
        todo!()
    }
}
