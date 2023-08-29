

use rand::{seq::SliceRandom, Rng};

use crate::game_functionality::{get_possible_moves, current_turn_is_crosses, make_move, determine_outcome, Outcome};

use super::{Brain, key::{position_to_key, key_to_pos}};

pub fn practise(brain: &mut Brain, neuron_key: &str) -> Outcome {
    let mut board = key_to_pos(neuron_key);

    while determine_outcome(&board) == Outcome::Unfinished {
        let mut rng = rand::thread_rng();
        // if rng.gen_range(1..=20) == 1 {
        //     print_board(&board);
        // }

        let possible_moves = get_possible_moves(&board);
        brain.add_any_new_neurons(&board, &possible_moves, neuron_key);
        let current_turn_is_crosses = current_turn_is_crosses(&board);
        // choose_most_exciting_move();
        if let Some((row, col)) = possible_moves.choose(&mut rng) {
            make_move(&mut board, *row, *col);
            if current_turn_is_crosses {
                brain.neurons_used_for_crosses.push(position_to_key(&board));
            } else {
                brain.neurons_used_for_noughts.push(position_to_key(&board));
            }
        } else {
            panic!("Vector is empty!");
        }
    }

    return determine_outcome(&board)
}