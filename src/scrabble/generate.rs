use std::collections::HashMap;

use super::{Game, MultiplierKind, Player, Tile, TileKind};

pub fn standard_game(players: &[Player]) -> Game {
    let bag_template: HashMap<char, u8> = HashMap::from([]);
    let board_template: HashMap<char, MultiplierKind> = HashMap::from([
        ('-', MultiplierKind::None),
        ('a', MultiplierKind::Letter(2)),
        ('b', MultiplierKind::Letter(3)),
        ('c', MultiplierKind::Word(2)),
        ('d', MultiplierKind::Word(3)),
    ]);

    let bag = bag(&[]);
    let board = board(
        15,
        &board_template,
        &[
            'd', '-', '-', 'a', '-', '-', '-', 'd', '-', '-', '-', 'a', '-', '-', 'd', '-', 'c',
            '-', '-', '-', 'b', '-', '-', '-', 'b', '-', '-', '-', 'c', '-', '-', '-', 'c', '-',
            '-', '-', 'a', '-', 'a', '-', '-', '-', 'c', '-', '-', 'a', '-', '-', 'c', '-', '-',
            '-', 'a', '-', '-', '-', 'c', '-', '-', 'a', '-', '-', '-', '-', 'c', '-', '-', '-',
            '-', '-', 'c', '-', '-', '-', '-', '-', 'b', '-', '-', '-', 'b', '-', '-', '-', 'b',
            '-', '-', '-', 'b', '-', '-', '-', 'a', '-', '-', '-', 'a', '-', 'a', '-', '-', '-',
            'a', '-', '-', 'd', '-', '-', 'a', '-', '-', '-', 'c', '-', '-', '-', 'a', '-', '-',
            'd', '-', '-', 'a', '-', '-', '-', 'a', '-', 'a', '-', '-', '-', 'a', '-', '-', '-',
            'b', '-', '-', '-', 'b', '-', '-', '-', 'b', '-', '-', '-', 'b', '-', '-', '-', '-',
            '-', 'c', '-', '-', '-', '-', '-', 'c', '-', '-', '-', '-', 'a', '-', '-', 'c', '-',
            '-', '-', 'a', '-', '-', '-', 'c', '-', '-', '-', '-', '-', 'c', '-', '-', '-', 'a',
            '-', 'a', '-', '-', '-', 'c', '-', '-', '-', 'c', '-', '-', '-', 'b', '-', '-', '-',
            'b', '-', '-', '-', 'c', '-', 'd', '-', '-', 'a', '-', '-', '-', 'd', '-', '-', '-',
            'a', '-', '-', 'd',
        ],
    )
    .unwrap();
    Game::new(bag, board, players.to_vec(), bag_template, board_template)
}

fn bag(symbol_counts: &[(char, usize)]) -> Vec<char> {
    symbol_counts
        .iter()
        .map(|(symbol, count)| vec![*symbol; *count])
        .flatten()
        .collect()
}
fn board(
    row_len: usize,
    template: &HashMap<char, MultiplierKind>,
    symbols: &[char],
) -> Option<Vec<Vec<Tile>>> {
    symbols.chunks_exact(row_len).fold(
        Some(Vec::new()),
        |board: Option<Vec<Vec<Tile>>>, row: &[char]| -> Option<Vec<Vec<Tile>>> {
            let mut board = board?;
            board.push(
                row.into_iter()
                    .map(|symbol| -> Option<Tile> {
                        Some(Tile {
                            symbol: *symbol,
                            kind: TileKind::Board(*template.get(symbol)?),
                        })
                    })
                    .collect::<Option<_>>()?,
            );
            Some(board)
        },
    )
}
