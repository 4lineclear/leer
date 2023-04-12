
use std::fmt::{Debug, Formatter, Result as fmtResult};


pub const BOARD_SIZE: usize = 15;

pub struct Game {
    pub board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self { board: 
            generate_board([
                ['♠','-','-','♡','-','-','-','♠','-','-','-','♡','-','-','♠'],
                ['-','♣','-','-','-','♢','-','-','-','♢','-','-','-','♣','-'],
                ['-','-','♣','-','-','-','♡','-','♡','-','-','-','♣','-','-'],
                ['♡','-','-','♣','-','-','-','♡','-','-','-','♣','-','-','♡'],
                ['-','-','-','-','♣','-','-','-','-','-','♣','-','-','-','-'],
                ['-','♢','-','-','-','♢','-','-','-','♢','-','-','-','♢','-'],
                ['-','-','♡','-','-','-','♡','-','♡','-','-','-','♡','-','-'],
                ['♠','-','-','♡','-','-','-','♣','-','-','-','♡','-','-','♠'],
                ['-','-','♡','-','-','-','♡','-','♡','-','-','-','♡','-','-'],
                ['-','♢','-','-','-','♢','-','-','-','♢','-','-','-','♢','-'],
                ['-','-','-','-','♣','-','-','-','-','-','♣','-','-','-','-'],
                ['♡','-','-','♣','-','-','-','♡','-','-','-','♣','-','-','-'],
                ['-','-','♣','-','-','-','♡','-','♡','-','-','-','♣','-','-'],
                ['-','♣','-','-','-','♢','-','-','-','♢','-','-','-','♣','-'],
                ['♠','-','-','♡','-','-','-','♠','-','-','-','♡','-','-','♠']
        ]) }
    }
}
// ♡, ♢, ♣, ♠
fn generate_board(char_board: [[char; BOARD_SIZE]; BOARD_SIZE]) -> [[Tile; BOARD_SIZE]; BOARD_SIZE]{
    let mut board: [[Tile; BOARD_SIZE]; BOARD_SIZE] = [[Tile::new_boardtile('-'); 15]; 15];

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = Tile::new_boardtile(char_board[i][j]);
        }
    }
    board
}
impl Debug for Game {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "Board:\n  ")?;
        for i in 1..16 {
            write!(f, "{:X} ", i)?;
        }
        writeln!(f)?;

        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{:X} ", i+1)?;
            for tile in row {
                write!(f, "{} ", tile.letter)?;
            }
            writeln!(f, "{:X}", i+1)?;
        }
        write!(f, "  ")?;
        
        for i in 1..16 {
            write!(f, "{:X} ", i)?;
        }
        writeln!(f)
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub letter: char,
    pub tile_type: TileType,
}

impl Tile {
    pub fn new_boardtile(symbol: char) -> Tile {
        Tile { 
            letter: symbol, 
            tile_type: TileType::BoardTile(Multiplier::from_char(symbol))
        }
    }

    pub fn new_playtile(letter: char) -> Tile{
        Tile { 
            letter, 
            tile_type: TileType::PlayTile( 
                match letter {
                    '*'                                                         => 0,
                    'A' | 'E' | 'I' | 'L' | 'N' | 'O' | 'R' | 'S' | 'T' | 'U'   => 1,
                    'D' | 'G'                                                   => 2,
                    'B' | 'C' | 'M' | 'P'                                       => 3,
                    'F' | 'H' | 'V' | 'W' | 'Y'                                 => 4,
                    'K'                                                         => 5,
                    'J' | 'X'                                                   => 8,
                    'Q' | 'Z'                                                   => 10,
                     _                                                          => panic!("{letter} is not a valid character, must be 'A-Z' or '*' ")
                }
            )
        }
    }
}

#[derive(Clone, Copy)]
pub enum Multiplier {
    OA, // One All
    DL, // Double Letter
    TL, // Triple Letter
    DW, // Double Word
    TW  // Triple Word
}

impl Multiplier {
    pub fn from_char(symbol: char) -> Multiplier{
        match symbol {
            '-'  => Multiplier::OA,
            '♡'  => Multiplier::DL,
            '♢'  => Multiplier::TL,
            '♣'  => Multiplier::DW,
            '♠'  => Multiplier::TW,
             _   => panic!("{symbol} not a valid character, must be one of: '-, ♡, ♢, ♣, ♠'")
        }
    }   
}
// #[derive(Debug)]
// struct MultiplierConvertError;
// impl Error for MultiplierConvertError { }
// impl Display for MultiplierConvertError {
//     fn fmt(&self, f: &mut Formatter) -> fmtResult {
//         write!(f, "Invalid Char used")
//     }
// }

#[derive(Clone, Copy)]
pub enum TileType {
    BoardTile(Multiplier),
    PlayTile(u8),
}
