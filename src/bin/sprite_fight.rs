use sprite_fight::{make_game_info, run_game};

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let game_info = make_game_info();

    run_game(game_info.0, game_info.1, game_info.2)
}
