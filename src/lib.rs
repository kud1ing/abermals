mod canvas;
mod canvas_dom_svg;
mod cell;
mod error;
mod game;
mod symbol;
mod symbols;

use crate::game::Game;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_webassembly() {
    let mut game = Game::new();
    let _ = game.run();
}
