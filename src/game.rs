use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Paragraph, Widget, canvas},
};

use crate::fruit::Fruit;
use crate::geometry::Direction;
use crate::snake::Snake;

pub struct Game {
    width: u32,
    height: u32,
    level: u32,
    snake: Snake,
    fruit: Fruit,
    poll_timeout: std::time::Duration,
    over: bool,
    should_exit: bool,
}

impl Game {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.over {
            self.draw(terminal)?;
            self.handle_keys()?;
            self.snake.update();
            self.check_collisions();
        }
        if !self.should_exit {
            self.draw(terminal)?;
            while let Event::Key(event) = crossterm::event::read()?
                && event.code != KeyCode::Char('q')
            {}
            self.should_exit = true;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_keys(&mut self) -> std::io::Result<()> {
        if crossterm::event::poll(self.poll_timeout)?
            && let Event::Key(event) = crossterm::event::read()?
        {
            match event.code {
                KeyCode::Up | KeyCode::Char('w') => self.snake.change_direction(Direction::Up),
                KeyCode::Down | KeyCode::Char('s') => self.snake.change_direction(Direction::Down),
                KeyCode::Left | KeyCode::Char('a') => self.snake.change_direction(Direction::Left),
                KeyCode::Right | KeyCode::Char('d') => {
                    self.snake.change_direction(Direction::Right)
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.should_exit = true;
                    self.stop();
                }
                _ => {}
            }
        };
        Ok(())
    }

    fn check_collisions(&mut self) -> bool {
        for (idx, point) in self.snake.positions.iter().enumerate() {
            if point.x < 0
                || point.x >= self.width as i32
                || point.y < 0
                || point.y >= self.height as i32
                || self.snake.positions[..idx].contains(point)
            {
                self.stop();
                return true;
            }
            if *point == self.fruit.position {
                self.level_up();
                return true;
            }
        }
        false
    }

    fn level_up(&mut self) {
        self.level += 1;
        let decrease = self.poll_timeout / 10;
        self.poll_timeout -= decrease;
        self.fruit = Fruit::random(self.width as i32, self.height as i32);
        self.snake.grow();
    }

    fn stop(&mut self) {
        self.over = true;
    }
}

impl Default for Game {
    fn default() -> Self {
        let (rows, cols) = crossterm::terminal::size().unwrap();

        let width = rows / 4;
        let height = cols / 2;

        let fruit = Fruit::random(width as i32, height as i32);

        Game {
            width: width as u32,
            height: height as u32,
            level: 1,
            snake: Snake::new(),
            fruit,
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
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
        let current_level = Line::from(format!(" Level: {} ", self.level));

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(keymap.left_aligned())
            .title_bottom(current_level.right_aligned());

        canvas::Canvas::default()
            .block(block)
            .x_bounds([0.0, self.width.into()])
            .y_bounds([0.0, self.height.into()])
            .paint(|ctx| {
                ctx.marker(symbols::Marker::Sextant);
                ctx.draw(&self.fruit);
                ctx.draw(&self.snake);
            })
            .render(area, buf);

        if self.over {
            let popup_area = area.centered(Constraint::Percentage(50), Constraint::Length(5));
            ratatui::widgets::Clear.render(popup_area, buf);
            Paragraph::new(vec![
                "Game over :(".into(),
                "".into(),
                Line::from("Press 'q' to exit.").style(Style::default().italic()),
            ])
            .centered()
            .block(
                Block::bordered()
                    .title(" Game Over ")
                    .border_style(Style::default().fg(Color::Red)),
            )
            .render(popup_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

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
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
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
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
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
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
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
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }

    #[test]
    fn collide_if_snake_hits_itself() {
        let positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
        };

        assert!(game.check_collisions());
        assert!(game.over);
    }

    #[test]
    fn does_not_collide_if_snake_does_not_hit_itself() {
        let positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 10, y: 10 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
        };

        assert!(!game.check_collisions());
        assert!(!game.over);
    }

    #[test]
    fn collide_if_snake_hits_fruit_but_does_not_stop_game() {
        let positions = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ];

        let mut snake = Snake::new();
        snake.positions = positions;

        let mut game = Game {
            width: 50,
            height: 50,
            level: 1,
            snake,
            fruit: Fruit {
                position: Point { x: 0, y: 1 },
                color: Color::Magenta,
            },
            poll_timeout: std::time::Duration::from_millis(100),
            over: false,
            should_exit: false,
        };

        assert!(game.check_collisions());
        assert!(!game.over);
        assert_eq!(game.level, 2);
    }

    #[test]
    fn level_up() {
        let mut game = Game::default();

        let start_fruit_position = game.fruit.position.clone();

        // Check initial state as a reference
        assert_eq!(game.level, 1);
        assert_eq!(game.poll_timeout, std::time::Duration::from_millis(100));
        assert_eq!(game.snake.positions.len(), 5);

        game.level_up();

        assert_eq!(game.level, 2);
        assert_eq!(game.poll_timeout, std::time::Duration::from_millis(90));
        assert_ne!(game.fruit.position, start_fruit_position); // This can fail if we're *very* unlucky
        assert_eq!(game.snake.positions.len(), 6);

        let second_fruit_position = game.fruit.position.clone();

        game.level_up();

        assert_eq!(game.level, 3);
        assert_eq!(game.poll_timeout, std::time::Duration::from_millis(81));
        assert_ne!(game.fruit.position, second_fruit_position); // This can fail if we're *very* unlucky
        assert_eq!(game.snake.positions.len(), 7);
    }
}
