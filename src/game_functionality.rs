

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Nought,
    Cross
}

#[derive(Debug, PartialEq, Clone)]
pub enum Outcome {
    CrossWins,
    NoughtWins,
    Draw,
    Unfinished
}

pub type Board = Vec<Vec<Option<Symbol>>>;

pub fn manifest_board() -> Vec<Vec<Option<Symbol>>> {
    return vec![
        vec![None,None,None],
        vec![None,None,None],
        vec![None,None,None],
    ]
}

pub fn get_possible_moves(board: &Board) -> Vec<(usize, usize)> {
    let mut possible_moves = Vec::new();
    let mut row_i = 0;
    for row in board {
        let mut col_i = 0;
        for tile in row {
            if tile.is_none() {
                possible_moves.push((row_i, col_i));
            }
            col_i += 1;
        }
        row_i += 1;
    }

    return possible_moves
}

#[test]
fn test_possible_moves() {
    let board = vec![
        vec![None, Some(Symbol::Cross), Some(Symbol::Nought)],
        vec![None, Some(Symbol::Nought), Some(Symbol::Nought)],
        vec![Some(Symbol::Cross), None, None],
    ];

    assert_eq!(get_possible_moves(&board), vec![(0, 0), (1, 0), (2, 1), (2, 2)])
}



pub fn current_turn_is_crosses(board: &Board) -> bool {
    let mut crosses_count = 0;
    let mut noughts_count = 0;
    for row in board {
        for tile in row {
            match tile {
                Some(Symbol::Cross) => crosses_count += 1,
                Some(Symbol::Nought) => noughts_count += 1,
                None => {}
            }
        }
    }

    return crosses_count == noughts_count
}

pub fn determine_outcome(board: &Board) -> Outcome {
    if board[0][0] == Some(Symbol::Cross) && board[0][1] == Some(Symbol::Cross) && board[0][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[1][0] == Some(Symbol::Cross) && board[1][1] == Some(Symbol::Cross) && board[1][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[2][0] == Some(Symbol::Cross) && board[2][1] == Some(Symbol::Cross) && board[2][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[0][0] == Some(Symbol::Cross) && board[1][0] == Some(Symbol::Cross) && board[2][0] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[0][1] == Some(Symbol::Cross) && board[1][1] == Some(Symbol::Cross) && board[2][1] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[0][2] == Some(Symbol::Cross) && board[1][2] == Some(Symbol::Cross) && board[2][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[0][0] == Some(Symbol::Cross) && board[1][1] == Some(Symbol::Cross) && board[2][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[2][0] == Some(Symbol::Cross) && board[1][1] == Some(Symbol::Cross) && board[0][2] == Some(Symbol::Cross) {
        return Outcome::CrossWins
    }
    if board[0][0] == Some(Symbol::Nought) && board[0][1] == Some(Symbol::Nought) && board[0][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[1][0] == Some(Symbol::Nought) && board[1][1] == Some(Symbol::Nought) && board[1][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[2][0] == Some(Symbol::Nought) && board[2][1] == Some(Symbol::Nought) && board[2][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[0][0] == Some(Symbol::Nought) && board[1][0] == Some(Symbol::Nought) && board[2][0] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[0][1] == Some(Symbol::Nought) && board[1][1] == Some(Symbol::Nought) && board[2][1] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[0][2] == Some(Symbol::Nought) && board[1][2] == Some(Symbol::Nought) && board[2][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[0][0] == Some(Symbol::Nought) && board[1][1] == Some(Symbol::Nought) && board[2][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }
    if board[2][0] == Some(Symbol::Nought) && board[1][1] == Some(Symbol::Nought) && board[0][2] == Some(Symbol::Nought) {
        return Outcome::NoughtWins
    }

    for row in board {
        for tile in row {
            if tile.is_none() {
                return Outcome::Unfinished
            }
        }
    }

    return Outcome::Draw
}

pub fn make_move(board: &mut Board, row: usize, col: usize) {
    board[row][col] = match current_turn_is_crosses(&board) {
        true => Some(Symbol::Cross),
        false => Some(Symbol::Nought),
    };
}