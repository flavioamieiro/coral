use crate::geometry::Point;

#[derive(Debug)]
pub struct Fruit {
    pub position: Point,
}

impl Fruit {
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let position = Point {
            x: rand::random_range(0..max_x),
            y: rand::random_range(0..max_y),
        };
        Self { position }
    }
}
