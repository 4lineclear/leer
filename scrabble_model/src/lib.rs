
pub mod error;
pub mod config;

use std::collections::HashSet;

use error::ParseError;
use derivative::Derivative;


pub fn standard_game() -> Game {
    let map = HashSet::new();
    // let a = Vec::from_iter(&map);
    Game {
        bag: map,
        board: Vec::new(),
        players: Vec::new(),
        set_words: Vec::new(),
    }
}

#[derive(Debug)]
pub struct Game {
    // Config progress
    pub bag: HashSet<Tile>,   // ✓
    pub board: Vec<Tile>,     //
    pub players: Vec<Player>, //
    pub set_words: Vec<Word>, //
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy, Derivative)]
pub enum Tile {
    Board(MultiplierKind),
    Letter(LetterValue),
}

impl Tile {
    pub fn new_letter(letter: char, points: u8, count: u8) -> Tile {
        Tile::Letter(LetterValue {
            letter,
            points,
            count,
        })
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MultiplierKind {
    None,
    Letter(u8),
    Word(u8),
}

impl MultiplierKind {
    fn from(src: &str, message: &str, multiplier: u8) -> Result<Self, ParseError> {
        match src {
            "None" => Ok(MultiplierKind::None),
            "Letter" => Ok(MultiplierKind::Letter(multiplier)),
            "Word" => Ok(MultiplierKind::Word(multiplier)),
            _ => Err(ParseError::InvalidMultiplier(src.to_string(), message.to_string()))
        }
    }
}

#[derive(Eq, Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Hash)]
pub struct LetterValue {
    letter: char,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    points: u8,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    count: u8,
}

#[derive(Debug)]
pub struct Player {}

#[derive(Debug)]
pub struct Word {}
