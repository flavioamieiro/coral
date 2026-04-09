use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Widget, canvas},
};

use crate::snake::{Direction, Snake};

pub struct Game {
    width: u32,
    height: u32,
    snake: Snake,
    tick_timeout: std::time::Duration,
    over: bool,
}

impl Game {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.over {
            self.handle_keys()?;
            self.draw(terminal)?;
            self.snake.update();
            self.check_collisions();
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_keys(&mut self) -> std::io::Result<()> {
        if crossterm::event::poll(self.tick_timeout)?
            && let Event::Key(event) = crossterm::event::read()?
        {
            match event.code {
                KeyCode::Up | KeyCode::Char('w') => self.snake.change_direction(Direction::Up),
                KeyCode::Down | KeyCode::Char('s') => self.snake.change_direction(Direction::Down),
                KeyCode::Left | KeyCode::Char('a') => self.snake.change_direction(Direction::Left),
                KeyCode::Right | KeyCode::Char('d') => {
                    self.snake.change_direction(Direction::Right)
                }
                KeyCode::Esc | KeyCode::Char('q') => self.stop(),
                _ => {}
            }
        };
        Ok(())
    }

    fn check_collisions(&mut self) -> bool {
        for point in &self.snake.positions {
            if point.x < 0
                || point.x >= self.width as i32
                || point.y < 0
                || point.y >= self.height as i32
            {
                self.stop();
                return true;
            }
        }
        false
    }

    fn stop(&mut self) {
        self.over = true;
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            width: 200,
            height: 200,
            snake: Snake::new(),
            tick_timeout: std::time::Duration::from_millis(100),
            over: false,
        }
    }
}

impl Widget for &mut Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Snake ".bold());
        let keymap = Line::from(vec![
            " a: left | ".into(),
            "s: down | ".into(),
            "w: up | ".into(),
            "d: right |".into(),
            "| q: quit ".into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(keymap.centered());

        canvas::Canvas::default()
            .block(block)
            .x_bounds([0.0, self.width.into()])
            .y_bounds([0.0, self.height.into()])
            .paint(|ctx| {
                ctx.marker(symbols::Marker::Sextant);
                for point in &self.snake.positions {
                    ctx.draw(&canvas::Rectangle::new(
                        point.x.into(),
                        point.y.into(),
                        1.0,
                        1.0,
                        Color::Magenta,
                    ));
                }
            })
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snake::Point;

    #[test]
    fn collide_if_snake_hits_left_wall() {
        let positions = vec![
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 200,
            height: 200,
            snake,
            tick_timeout: std::time::Duration::from_millis(100),
            over: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }

    #[test]
    fn collide_if_snake_hits_bottom_wall() {
        let positions = vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 200,
            height: 200,
            snake,
            tick_timeout: std::time::Duration::from_millis(100),
            over: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }

    #[test]
    fn collide_if_snake_hits_right_wall() {
        let positions = vec![
            Point { x: 198, y: 1 },
            Point { x: 199, y: 1 },
            Point { x: 200, y: 1 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 200,
            height: 200,
            snake,
            tick_timeout: std::time::Duration::from_millis(100),
            over: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }

    #[test]
    fn collide_if_snake_hits_top_wall() {
        let positions = vec![
            Point { x: 0, y: 198 },
            Point { x: 0, y: 199 },
            Point { x: 0, y: 200 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 200,
            height: 200,
            snake,
            tick_timeout: std::time::Duration::from_millis(100),
            over: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }
}
