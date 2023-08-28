use crate::game_functionality::{Board, Symbol, manifest_board};

pub fn position_to_key(board: &Board) -> String {
    let mut key = "".to_string();
    for row_i in 0..3 {
        for col_i in 0..3 {
            match board[row_i][col_i] {
                Some(Symbol::Nought) => key.push('2'),
                Some(Symbol::Cross) => key.push('1'),
                None => key.push('0'),
            }
        }
    }

    return key
}

#[test]
fn test_pos_to_key() {
    let position = vec![
        vec![None, Some(Symbol::Cross), Some(Symbol::Nought)],
        vec![None, Some(Symbol::Nought), Some(Symbol::Nought)],
        vec![Some(Symbol::Cross), None, None],
    ];

    assert_eq!(position_to_key(&position), "012022100".to_string())
}

pub fn key_to_pos(key: &str) -> Board {
    let mut board = manifest_board();

    let mut row_i = 0;
    let mut col_i = 0;
    for char in key.chars() {
        board[row_i][col_i] = match char {
            '0' => None,
            '1' => Some(Symbol::Cross),
            '2' => Some(Symbol::Nought),
            _ => panic!("Unknown char in code")
        };

        if col_i != 2 {
            col_i += 1;
        } else {
            row_i += 1;
            col_i = 0;
        }
    }

    return board
}

#[test]
fn test_key_to_pos() {
    let key = "012022100";

    assert_eq!(key_to_pos(key), vec![
        vec![None, Some(Symbol::Cross), Some(Symbol::Nought)],
        vec![None, Some(Symbol::Nought), Some(Symbol::Nought)],
        vec![Some(Symbol::Cross), None, None],
    ])
}