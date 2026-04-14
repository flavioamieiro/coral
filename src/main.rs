mod game;
mod geometry;
mod snake;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| game::Game::default().run(terminal))
}
