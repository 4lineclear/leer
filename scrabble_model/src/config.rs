use std::{
    char::ParseCharError,
    collections::HashMap    ,
    fmt::Debug,
    fs::{self},
    io::{self},
    num::ParseIntError, str::FromStr,
};

use thiserror::Error;

use crate::{letter_pair, LetterValue, Tile};

const BAG_PATH: &str = "bag.csv";

pub fn load() -> Result<(), ParseError> {
    load_bag()?;
    Ok(())
    // let mut path = current_exe()?;

    // path.pop();
    // path.push("config");

    // fs::read_dir(path)
}

fn load_bag() -> Result<HashMap<Tile, LetterValue>, ParseError> {
    let file_data = fs::read_to_string(BAG_PATH)?;
    file_data
        .lines()
        .enumerate()
        .map(load_letter)
        .collect::<Result<HashMap<Tile, LetterValue>, ParseError>>()
}

fn load_letter(val: (usize, &str)) -> Result<(Tile, LetterValue), ParseError> {
    let row: Vec<&str> = val.1.split(',').map(str::trim).collect();

    if row.len() == 3 {
        Ok(letter_pair(
            parse(val.0, row[0], ParseError::InvalidChar)?,
            parse(val.0, row[1], ParseError::InvalidU8)?,
            parse(val.0, row[2], ParseError::InvalidU8)?,
        ))
    } else {
        Err(ParseError::Bag(BagError::InvalidRowLength(
            row.len(),
            format!("{}:{}", BAG_PATH, val.0),
        )))
    }
}


fn parse<T: FromStr, F>(line: usize, src: &str, f: F) -> Result<T, ParseError>
where
    F: Fn(<T as FromStr>::Err, String) -> ParseError,
{
    match src.parse::<T>() {
        Ok(n) => Ok(n),
        Err(e) => Err( f(e, format!("{}:{}", BAG_PATH, line) )),
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    Bag(#[from] BagError),
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("...")]
    InvalidChar(ParseCharError, String),

    #[error("...")]
    InvalidU8(ParseIntError, String),
}

#[derive(Error, Debug)]
pub enum BagError {
    #[error("...")]
    InvalidRowLength(usize, String),
}

// impl Debug for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(self.to_string().as_str())
//     }
// }
