use rand::seq::SliceRandom;

use crate::{game_functionality::{get_possible_moves, current_turn_is_crosses, Board, make_move, determine_outcome, Outcome}, game_interface::print_board};

use super::{Brain, Neuron, key::{position_to_key, key_to_pos}};

pub fn practise(brain: &mut Brain, neuron_key: &str) -> Outcome {
    let mut board = key_to_pos(neuron_key);

    while determine_outcome(&board) == Outcome::Unfinished {
        print_board(&board);
        let possible_moves = get_possible_moves(&board);
        add_any_new_neurons(brain, &board, &possible_moves, neuron_key);
        let mut rng = rand::thread_rng();
        let current_turn_is_crosses = current_turn_is_crosses(&board);
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

fn add_any_new_neurons(
    brain: &mut Brain,
    board: &Board,
    possible_moves: &Vec<(usize, usize)>,
    parent_key: &str
) {
    for (row, col) in possible_moves {
        let mut board = board.clone();
        make_move(&mut board, *row, *col);
        let key = position_to_key(&board);
        if brain.neurons.get(&key).is_none() {
            brain.neurons.insert(key.clone(), Neuron::manifest(Some(parent_key.to_string())));
        }

        let parent = brain.neurons.get_mut(parent_key).unwrap();
        parent.children_neurons.push(key);
    }
}