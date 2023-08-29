

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rand::{seq::SliceRandom, Rng};

use crate::{game_functionality::{get_possible_moves, current_turn_is_crosses, make_move, determine_outcome, Outcome, Board}, game_interface::print_board};

use super::{Brain, key::{position_to_key, key_to_pos}, Neuron};

pub fn practise(brain: &mut Brain, neuron_key: &str, attempt_number: usize) -> Outcome {
    let mut board = key_to_pos(neuron_key);
    brain.remember_neuron_used(&board, !current_turn_is_crosses(&board));

    let mut move_count = 0;
    while determine_outcome(&board) == Outcome::Unfinished {
        let mut rng = rand::thread_rng();
        // if rng.gen_range(1..=20) == 1 {
        //     print_board(&board);
        // }

        let possible_moves = get_possible_moves(&board);
        brain.add_any_new_neurons(&board, &possible_moves, neuron_key);

        let (row, col) = {
            if move_count < 3 && attempt_number < 50 {
                if let Some((row, col)) = possible_moves.choose(&mut rng) {
                    (*row, *col)
                } else {
                    panic!("No moves!")
                }
            } else {
                brain.choose_most_exciting_move(&possible_moves, &board)
            }
        };
        make_move(&mut board, row, col);
        // print_board(&board);
        // thread::sleep(Duration::from_secs(2));

        brain.remember_neuron_used(&board, !current_turn_is_crosses(&board));
        move_count += 1;
    }

    return determine_outcome(&board)
}

impl Brain {
    fn add_any_new_neurons(
        &mut self,
        board: &Board,
        possible_moves: &Vec<(usize, usize)>,
        parent_key: &str
    ) {
        let mut neurons = self.neurons.lock().unwrap();
        for (row, col) in possible_moves {
            let mut board = board.clone();
            make_move(&mut board, *row, *col);
            let key = position_to_key(&board);
            let neuron = Arc::new(Mutex::new(
                Neuron::manifest(Some(
                    neurons.get(parent_key).unwrap().clone()
                ))
            ));
            if neurons.get(&key).is_none() {
                neurons.insert(key.clone(), neuron.clone());
            }

            let parent = neurons.get_mut(parent_key).unwrap();
            parent.lock().unwrap().children_neurons.push(neuron.clone());
        }
    }

    fn choose_most_exciting_move(
        &self,
        possible_moves: &Vec<(usize, usize)>,
        board: &Board
    ) -> (usize, usize) {

        let mut board_clone = board.clone();
        make_move(&mut board_clone, possible_moves[0].0, possible_moves[0].1);

        let neurons = self.neurons.lock().unwrap();
        let most_excited_neuron_key = position_to_key(&board_clone);
        let mut most_excited_neuron = neurons.get(&most_excited_neuron_key).unwrap().clone();
        let mut most_exciting_move = possible_moves[0];
        for (row, col) in possible_moves {
            let mut board_clone = board.clone();
            make_move(&mut board_clone, *row, *col);
            let current_key = position_to_key(&board_clone);
            let current_neuron = neurons.get(&current_key).unwrap().clone();

            if most_excited_neuron_key != current_key {
                if current_neuron.lock().unwrap().upper_confidence_value(self)
                > most_excited_neuron.lock().unwrap().upper_confidence_value(self) {
                    most_excited_neuron = current_neuron;
                    most_exciting_move = (*row, *col);
                }
            }
        }
        return most_exciting_move
    }

    fn remember_neuron_used(&mut self, board: &Board, current_turn_is_crosses: bool) {
        if current_turn_is_crosses {
            self.neurons_used_for_crosses.push(position_to_key(&board));
        } else {
            self.neurons_used_for_noughts.push(position_to_key(&board));
        }
    }
}