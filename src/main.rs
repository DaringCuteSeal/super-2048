use std::process::exit;

use crossterm::terminal::disable_raw_mode;

mod game_data;
mod game_io;
mod merging;

fn exit_msg(game: &game_data::GameData) {
    println!("Your score: {}", game.score);
}

fn main() {
    let mut game = game_data::GameData::new();
    game.insert_random();
    while game.has_legal_moves() {
        game_io::render(&game);
        let user_move: game_io::InputType;
        loop {
            if let Some(i) = game_io::get_input() {
                user_move = i;
                break;
            }
        }

        match user_move {
            game_io::InputType::UP => {
                game.gravity_up();
            }

            game_io::InputType::DOWN => {
                game.gravity_down();
            }

            game_io::InputType::LEFT => {
                game.gravity_left();
            }

            game_io::InputType::RIGHT => {
                game.gravity_right();
            }

            game_io::InputType::EXIT => {
                disable_raw_mode().unwrap();
                exit_msg(&game);
                exit(0)
            }
        }
        game.insert_random();
    }

    game_io::render(&game);
    println!("Game over, no moves left!");
    exit_msg(&game);
}
