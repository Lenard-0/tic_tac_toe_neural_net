use crate::{game_functionality::{Board, Symbol, manifest_board, determine_outcome, Outcome}, ai_functionality::Brain};
use std::io;

pub fn play_game(genius_ai: Brain) {
    let mut board = manifest_board();

    while determine_outcome(&board) == Outcome::Unfinished {
        let mut eligible_move = None;
        while eligible_move.is_none() {
            println!("Please input your move: ie top left corner is 00, middle is 11");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let m = match input_to_move(input.trim().chars().collect()) {
                Ok(m) => m,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
            if is_possible_move(m, &board) {
                eligible_move = Some(m);
            } else {
                println!("Move: {:?} is not available", m);
            }
        }
        board[eligible_move.unwrap().0][eligible_move.unwrap().1] = Some(Symbol::Cross);
        print_board(&board);

        if determine_outcome(&board) != Outcome::Unfinished {
            break;
        }

        genius_ai.choose_best_move(&mut board);
        print_board(&board);
    }

    match determine_outcome(&board) {
        Outcome::NoughtWins => println!("I won! Better luck next time."),
        Outcome::CrossWins => println!("You won! I need to practise harder next time."),
        _ => println!("It was a draw! I need to practise harder next time."),
    }
}

pub fn print_board(board: &Board) {
    println!("  {}  |  {}  |  {}  ", &symbol_to_string(&board[0][0]), &symbol_to_string(&board[0][1]), &symbol_to_string(&board[0][2]));
    println!("-----|-----|-----");
    println!("  {}  |  {}  |  {}  ", &symbol_to_string(&board[1][0]), &symbol_to_string(&board[1][1]), &symbol_to_string(&board[1][2]));
    println!("-----|-----|-----");
    println!("  {}  |  {}  |  {}  ", &symbol_to_string(&board[2][0]), &symbol_to_string(&board[2][1]), &symbol_to_string(&board[2][2]));
}

fn symbol_to_string(symbol: &Option<Symbol>) -> String {
    return match symbol {
        Some(Symbol::Cross) => "x".to_string(),
        Some(Symbol::Nought) => "o".to_string(),
        None => " ".to_string(),
    }
}

fn input_to_move(input: Vec<char>) -> Result<(usize, usize), String> {
    if input.len() != 2 {
        return Err("Error: Your move should only contain 2 characters!".to_string())
    }

    println!("input: {:?}", input);

    if !(input[0] == '0' || input[0] == '1' || input[0] == '2')
    || !(input[1] == '0' || input[1] == '1' || input[1] == '2') {
        return Err("Error: Your move should only consist of characters 0, 1, or 2!".to_string())
    }
    return Ok((input[0].to_digit(10).unwrap() as usize, input[1].to_digit(10).unwrap() as usize))
}

fn is_possible_move(m: (usize, usize), board: &Board) -> bool {
    return match board[m.0][m.1] {
        Some(_) => false,
        None => true
    }
}