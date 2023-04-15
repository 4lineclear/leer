use std::fmt::{Debug, Formatter, Result as fmtResult};

mod generation;

pub const BOARD_SIZE: usize = 15;

pub struct Game {
    pub board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
    bag: Vec<char>
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self { board: generation::generate_board(), bag: generation::generate_bag() }
    }
}
// ♡, ♢, ♣, ♠
impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Board:\n ")?;
        for i in 1..16 {
            write!(f, "{:X} ", i)?;
        }
        writeln!(f)?;

        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{:X} ", i + 1)?;
            for tile in row {
                write!(f, "{} ", tile.symbol)?;
            }
            writeln!(f, "{:X}", i + 1)?;
        }
        write!(f, "  ")?;

        for i in 1..16 {
            write!(f, "{:X} ", i)?;
        }
        write!(f,"\nBag:\n")?;
        
        for letter in &self.bag{
            write!(f, "{letter}, ")?;
        }
        write!(f,"")
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub symbol: char,
    pub tile_type: TileType,
}

impl Tile {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            tile_type: TileType::new(symbol),
        }
    }
}

// impl FromIterator<Tile> for Tile {
//     fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
//         todo!()
//     }
// }

#[derive(Clone, Copy)]
pub enum TileType {
    BoardTile(Multiplier),
    PlayTile(u8),
}

impl TileType {
    fn new(symbol: char) -> Self {
        match symbol {
            '-'                             => TileType::BoardTile(Multiplier::OA),
            '♡'                             => TileType::BoardTile(Multiplier::DL),
            '♢'                             => TileType::BoardTile(Multiplier::TL),
            '♣' | '★'                       => TileType::BoardTile(Multiplier::DW),
            '♠'                             => TileType::BoardTile(Multiplier::TW),

            '*'                             => TileType::PlayTile(0),
            'A' | 'E' | 'I' | 'L' | 'N' | 
            'O' | 'R' | 'S' | 'T' | 'U'     => TileType::PlayTile(1),
            'D' | 'G'                       => TileType::PlayTile(2),
            'B' | 'C' | 'M' | 'P'           => TileType::PlayTile(3),
            'F' | 'H' | 'V' | 'W' | 'Y'     => TileType::PlayTile(4),
            'K'                             => TileType::PlayTile(5),
            'J' | 'X'                       => TileType::PlayTile(8),
            'Q' | 'Z'                       => TileType::PlayTile(10),

            _ => panic!("{symbol}"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Multiplier {
    OA, // One All
    DL, // Double Letter
    TL, // Triple Letter
    DW, // Double Word
    TW, // Triple Word
}

// #[derive(Debug)]
// struct MultiplierConvertError;
// impl Error for MultiplierConvertError { }
// impl Display for MultiplierConvertError {
//     fn fmt(&self, f: &mut Formatter) -> fmtResult {
//         write!(f, "Invalid Char used")
//     }
// }