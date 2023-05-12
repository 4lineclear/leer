pub mod config;
pub mod error;

use std::collections::{HashMap, HashSet};

use derivative::Derivative;
use error::ParseError;
use serde::{Deserialize, Serialize};

use crate::error::Result;

pub fn standard_game() -> Game {
    let map = HashSet::new();
    Game {
        bag: map,
        board: Vec::new(),
        players: Vec::new(),
        set_words: Vec::new(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    // Config progress
    pub bag: HashSet<Tile>,   // ✓
    pub board: Vec<Tile>,     // ✓
    pub players: Vec<Player>, // ✓
    pub set_words: Vec<Word>, // ✓
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy, Derivative, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Derivative, Serialize, Deserialize)]
pub struct MultiplierBuilder {
    template: HashMap<String, MultiplierKind>,
}

impl MultiplierBuilder {
    pub fn new(template: HashMap<String, MultiplierKind>) -> Self {
        Self { template }
    }

    pub fn build(&self, src: &str) -> Result<&MultiplierKind> {
        self.template
            .get(src)
            .ok_or(ParseError::InvalidMultiplier(src.to_string()))
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MultiplierKind {
    None,
    Letter(u8),
    Word(u8),
}

#[derive(Eq, Debug, Clone, Copy, Derivative, Serialize, Deserialize)]
#[derivative(PartialEq, Hash)]
pub struct LetterValue {
    letter: char,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    points: u8,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    count: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {}
