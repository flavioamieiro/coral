use ratatui::DefaultTerminal;

pub struct Game {}

impl Game {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal
                .draw(|frame| frame.render_widget("Hi! Press any key to exit", frame.area()))?;

            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    }
}
