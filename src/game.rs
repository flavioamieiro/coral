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

use crate::snake::Snake;

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
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_keys(&mut self) -> std::io::Result<()> {
        if crossterm::event::poll(self.tick_timeout)? {
            if crossterm::event::read()?.is_key_press() {
                self.stop();
            };
        };
        Ok(())
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
