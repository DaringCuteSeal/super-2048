mod game_data;
mod game_io;
mod merging;

fn main() {
    let mut game = game_data::GameData::new();
    game.insert_random();
    while game.has_legal_moves() {
        game_io::render(&game);
        let user_move: game_io::Move;
        loop {
            if let Some(i) = game_io::get_input() {
                user_move = i;
                break;
            }
        }

        match user_move {
            game_io::Move::UP => {
                game.gravity_up();
            }

            game_io::Move::DOWN => {
                game.gravity_down();
            }

            game_io::Move::LEFT => {
                game.gravity_left();
            }

            game_io::Move::RIGHT => {
                game.gravity_right();
            }
        }
        game.insert_random();
    }

    game_io::render(&game);
    println!("Game over, no moves left!");
}
