mod game;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut game = game::Game {};
        game.run(terminal)
    })
}
