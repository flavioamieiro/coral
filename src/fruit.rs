use rand::seq::IndexedRandom;
use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::geometry::{FilledRectangle, Point};

#[derive(Debug)]
pub struct Fruit {
    pub position: Point,
    pub color: Color,
}

impl Fruit {
    pub fn random(max_x: i32, max_y: i32) -> Self {
        let position = Point {
            x: rand::random_range(0..max_x),
            y: rand::random_range(0..max_y),
        };
        let mut rng = rand::rng();
        let color = *[Color::Magenta, Color::Green, Color::Yellow]
            .choose(&mut rng)
            .unwrap();
        Self { position, color }
    }
}

impl Shape for Fruit {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        FilledRectangle::new(
            self.position.x.into(),
            self.position.y.into(),
            1.0,
            1.0,
            0.1,
            self.color,
        )
        .draw(painter);
    }
}
