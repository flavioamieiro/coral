use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Rectangle, Shape},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn conflicts(&self, other: &Self) -> bool {
        match self {
            Self::Up => other == &Self::Down,
            Self::Down => other == &Self::Up,
            Self::Left => other == &Self::Right,
            Self::Right => other == &Self::Left,
        }
    }
}

pub struct FilledRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub fill_step: f64,
    pub color: Color,
}

impl FilledRectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64, fill_step: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill_step,
            color,
        }
    }
}

impl Shape for FilledRectangle {
    fn draw(&self, painter: &mut Painter) {
        let subdivisions: i32 = (self.height / self.fill_step).round() as i32;
        for y_off in (1..subdivisions).map(|y| y as f64 * self.fill_step) {
            let this_y = self.y + y_off;
            Rectangle::new(self.x, this_y, self.width, self.fill_step, self.color).draw(painter);
        }
    }
}
