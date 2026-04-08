mod game;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| game::Game::default().run(terminal))
}
