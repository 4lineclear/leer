pub mod generate;

use std::{cell::Cell, collections::HashMap};

#[derive(Debug)]
pub struct Game {
    pub bag: Vec<char>,
    pub board: Vec<Vec<Tile>>,
    pub players: Vec<Player>,
    pub current_player: Cell<u8>,
    /// contains the points a symbol holds
    bag_template: HashMap<char, u8>,
    /// contains the multiplier a symbol holds
    board_template: HashMap<char, MultiplierKind>,
}

impl Game {
    pub fn new(
        bag: Vec<char>,
        board: Vec<Vec<Tile>>,
        players: Vec<Player>,
        bag_template: HashMap<char, u8>,
        board_template: HashMap<char, MultiplierKind>,
    ) -> Self {
        Self {
            bag,
            board,
            players,
            current_player: Cell::new(0),
            bag_template,
            board_template,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    symbol: char,
    kind: TileKind,
}

#[derive(Debug, Clone, Copy)]
pub enum TileKind {
    /// Represents without a tile letter place on yet
    Board(MultiplierKind),
    /// Represents a played tile
    Play,
}

#[derive(Debug, Clone, Copy)]
pub enum MultiplierKind {
    None,
    Word(u8),
    Letter(u8),
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub points: Cell<u8>,
}
