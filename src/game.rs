use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{canvas, Block, Widget},
    DefaultTerminal,
};

pub struct Game {
    width: u32,
    height: u32,
}

impl Game {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            self.draw(terminal)?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            width: 200,
            height: 200,
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

        let snake_pos = vec![
            (85.0, 90.0),
            (86.0, 90.0),
            (87.0, 90.0),
            (88.0, 90.0),
            (89.0, 90.0),
            (90.0, 90.0),
            (90.0, 91.0),
            (90.0, 92.0),
            (90.0, 93.0),
            (90.0, 94.0),
            (90.0, 95.0),
            (90.0, 96.0),
            (90.0, 97.0),
            (90.0, 98.0),
            (90.0, 99.0),
            (91.0, 99.0),
            (92.0, 99.0),
            (93.0, 99.0),
            (94.0, 99.0),
            (95.0, 99.0),
            (96.0, 99.0),
        ];

        canvas::Canvas::default()
            .block(block)
            .x_bounds([0.0, self.width.into()])
            .y_bounds([0.0, self.height.into()])
            .paint(|ctx| {
                ctx.marker(symbols::Marker::Sextant);
                for (x, y) in &snake_pos {
                    ctx.draw(&canvas::Rectangle::new(*x, *y, 1.0, 1.0, Color::Magenta));
                }
            })
            .render(area, buf);
    }
}
