use serde_json::{json, Value};

use log::info;

use crate::{engine::Engine, Battlesnake, Board, Game};

pub fn get_info() -> Value {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "cogsandsquigs",
        "color": "#111827",
        "head": "sand-worm",
        "tail": "sharp",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, _board: &Board, you: &Battlesnake) -> &'static str {
    let engine = Engine::new();
    engine.get_next_move(game, _turn, _board, you)
}
