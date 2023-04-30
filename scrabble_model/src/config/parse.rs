use crate::{MultiplierKind, Tile, error::*};
use std::collections::{HashMap, HashSet};
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

// Format:
// <letter>, <points>, <count>
const BAG_PATH: &str = "bag.csv";
// Format
// <letter, kind, value>...
const BOARD_CONFIG_PATH: &str = "board_config.csv";
// Format
// <kind><space>
const BOARD_LAYOUT_PATH: &str = "board.csv";

use crate::error::ParseError::{self, *};
pub(crate) fn parse_bag() -> Result<HashSet<Tile>> {
    let mut original_length = 0;
    let file_data = lines_from_file(BAG_PATH)?.flatten();

    let bag = file_data
        .enumerate()
        .inspect(|_| original_length += 1)
        .map(parse_bag_row)
        .collect::<Result<HashSet<Tile>>>()?;

    if original_length == bag.len() {
        Ok(bag)
    } else {
        Err(BagDuplicate())
    }
}

fn parse_bag_row(val: (usize, String)) -> Result<Tile> {
    let (line, row) = (val.0 + 1, val.1);
    let row: Vec<&str> = row.split(',').map(str::trim).collect();

    if row.len() == 3 {
        Ok(Tile::new_letter(
            parse_from_str(line, row[0], InvalidChar, BAG_PATH)?,
            parse_from_str(line, row[1], InvalidU8, BAG_PATH)?,
            parse_from_str(line, row[2], InvalidU8, BAG_PATH)?,
        ))
    } else {
        Err(InvalidRowLength(row.len(), fmt_path(BAG_PATH, line)))
    }
}

pub(crate) fn parse_board() -> Result<Vec<Tile>> {
    let mapping = |val: (usize, String)| parse_board_row(val, parse_board_config()?);

    Ok(lines_from_file(BOARD_LAYOUT_PATH)?
        .flatten()
        .enumerate()
        .map(mapping)
        .flatten()
        .flatten()
        .collect::<Vec<_>>())
}

fn parse_board_row(
    val: (usize, String),
    letter_map: HashMap<char, MultiplierKind>,
) -> Result<Vec<Tile>> {
    let (line, row) = (val.0 + 1, val.1);
    let parse_unit = |s: &str| -> Result<Tile> { parse_board_tile(line, s, &letter_map) };
    Ok(row
        .split(' ')
        .map(parse_unit)
        .flatten()
        .collect::<Vec<Tile>>())
}

fn parse_board_tile(
    line: usize,
    s: &str,
    letter_map: &HashMap<char, MultiplierKind>,
) -> Result<Tile> {
    Ok(Tile::Board(
        *letter_map
            .get(&parse_from_str(line, s, InvalidChar, BOARD_CONFIG_PATH)?)
            .ok_or(CharNotAllowed(BOARD_CONFIG_PATH.to_string()))?,
    ))
}

fn parse_board_config() -> Result<HashMap<char, MultiplierKind>> {
    let mut original_length = 0;
    lines_from_file(BOARD_CONFIG_PATH)?
        .flatten()
        .enumerate()
        .inspect(|_| original_length += 1)
        .map(parse_board_config_row)
        .collect::<Result<HashMap<char, MultiplierKind>>>()
}

fn parse_board_config_row(val: (usize, String)) -> Result<(char, MultiplierKind)> {
    let (line, row) = (val.0 + 1, val.1);
    let row: Vec<&str> = row.split(',').map(str::trim).collect();

    if row.len() == 3 {
        let letter: char = parse_from_str(line, row[1], InvalidChar, BOARD_CONFIG_PATH)?;
        let multiplier: u8 = parse_from_str(line, row[2], InvalidU8, BOARD_CONFIG_PATH)?;

        Ok((
            letter,
            MultiplierKind::from(row[0], BOARD_CONFIG_PATH, multiplier)?,
        ))
    } else {
        Err(InvalidRowLength(
            row.len(),
            fmt_path(BOARD_CONFIG_PATH, line),
        ))
    }
}
fn parse_from_str<T: FromStr, F>(line: usize, src: &str, f: F, path: &str) -> Result<T>
where
    F: Fn(<T as FromStr>::Err, String) -> ParseError,
{
    match src.parse::<T>() {
        Ok(t) => Ok(t),
        Err(e) => Err(f(e, fmt_path(path, line))),
    }
}

fn lines_from_file(path: &str) -> Result<Lines<BufReader<File>>> {
    match File::open(path) {
        Ok(f) => Ok(BufReader::new(f).lines()),
        Err(e) => Err(Io(e)),
    }
}

fn fmt_path(path: impl Display, line: usize) -> String {
    format!("{}:{}", path, line)
}
