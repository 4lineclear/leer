use crate::{standard_game, Game, error::Result};

mod parse;


pub fn load() -> Result<Game> {
    let bag = parse::parse_bag()?;
    let board =parse::parse_board()?;

    Ok(Game {
        bag,
        board,
        ..standard_game()
    })
}