use macroquad::prelude::*;

mod game;
mod grid;
mod particle;
mod snake;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 900,
        window_height: 900,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(Color::from_rgba(18, 18, 18, 255));

        game.update();

        game.draw();

        next_frame().await;
    }
}