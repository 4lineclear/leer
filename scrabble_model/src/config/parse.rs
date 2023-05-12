use csv::ReaderBuilder;

use crate::{error::*, LetterValue, MultiplierBuilder, Tile};
use std::{collections::HashSet, fs};

// Format:
// <letter>, <points>, <count>
const BAG_PATH: &str = "bag.csv";

// Format
// <letter, kind, value>...
const BOARD_TEMPLATE_PATH: &str = "board_template.csv";

// Format
// <kind><space>
const BOARD_LAYOUT_PATH: &str = "board.csv";

pub(crate) fn bag() -> Result<HashSet<Tile>> {
    Ok(ReaderBuilder::new()
        .has_headers(false)
        .from_path(BAG_PATH)?
        .deserialize::<LetterValue>()
        .flatten()
        .map(|src| Tile::Letter(src))
        .collect::<HashSet<Tile>>())
}

pub(crate) fn board() -> Result<Vec<Tile>> {
    let file = fs::read_to_string(BOARD_TEMPLATE_PATH)?;
    let builder: MultiplierBuilder = serde_json::from_str(&file)?;

    let mut tiles: Vec<Tile> = Vec::new();

    for line in fs::read_to_string(BOARD_LAYOUT_PATH)?.lines() {
        for src in line.split(' ') {
            tiles.push(Tile::Board(*builder.build(&src)?));
        }
    }
    Ok(tiles)
}
