use self::collision::Ball;

mod collision;
mod constants;

struct Table {
    balls: Vec<Ball>,
}
