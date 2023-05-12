use crate::{error::Result, Game};

mod parse;

pub fn load() -> Result<Game> {
    let bag = parse::bag()?;
    let board = parse::board()?;

    Ok(Game {
        bag,
        board,
        players: Vec::new(),
        set_words: Vec::new(),
    })
}
