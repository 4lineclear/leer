
pub mod config;

use std::collections::HashMap;

pub fn standard_game() -> Game {
    let map = HashMap::new();
    dbg!(&map);
    Game {
        bag: map,
        board: Vec::new(),
        players: Vec::new(),
        set_words: Vec::new(),
    }
}

#[derive(Debug)]
pub struct Game {
    bag: HashMap<Tile, LetterValue>,
    board: Vec<Tile>,
    players: Vec<Player>,
    set_words: Vec<Word>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Tile {
    Board { multiplier: u8 },
    Letter { letter: char },
}

#[derive(Debug)]
pub(crate) struct LetterValue {
    points: u8,
    count: u8,
}

pub(crate) fn letter_pair(letter: char, points: u8, count: u8) -> (Tile, LetterValue) {
    (Tile::Letter { letter }, LetterValue { points, count })
}

#[derive(Debug)]
pub struct Player {}

#[derive(Debug)]
pub struct Word {}
