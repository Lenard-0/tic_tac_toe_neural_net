use std::collections::HashMap;
use crate::game_functionality::{Board, get_possible_moves, make_move, Symbol};
use std::sync::{Arc, RwLock};
use std::thread;
use self::{practise::practise, backpropagate::backpropagate, key::position_to_key};

pub mod key;
pub mod practise;
pub mod backpropagate;

#[derive(Debug, Clone)]
pub struct Brain {
    neurons: HashMap<String, Neuron>,
    neurons_used_for_crosses: Vec<String>,
    neurons_used_for_noughts: Vec<String>,
}

impl Brain {
    pub fn manifest() -> Self {
        let mut neurons = HashMap::new();
        neurons.insert("000000000".to_string(), Neuron::manifest(None));
        return Brain { neurons, neurons_used_for_crosses: vec![], neurons_used_for_noughts: vec![] }
    }

    pub fn choose_move(&self, board: &mut Board) {
        let possible_moves = get_possible_moves(board);
        let mut best_neuron = Neuron::activate_neuron(board.clone(), possible_moves[0], self);
        let mut best_move = possible_moves[0];
        for m in possible_moves {
            let current_neuron = Neuron::activate_neuron(board.clone(), m, self);
            if current_neuron.potential() > best_neuron.potential() {
                best_neuron = current_neuron;
                best_move = m;
            }
        }

        board[best_move.0][best_move.1] = Some(Symbol::Nought)
    }
}

#[derive(Debug, Clone)]
pub struct Neuron {
    visit_count: usize,
    win_count: usize,
    parent_neuron: Option<String>,
    children_neurons: Vec<String>
}

impl Neuron {
    pub fn manifest(parent_neuron: Option<String>) -> Self {
        return Neuron {
            visit_count: 0,
            win_count: 0,
            parent_neuron,
            children_neurons: vec![]
        }
    }

    pub fn get_most_excited(brain: &Brain) -> (Self, String) {
        let mut least_visited_neuron = brain.neurons.get("000000000").unwrap();
        let mut least_visited_neuron_key = "000000000";
        for (neuron_key, neuron) in &brain.neurons {
            if neuron.visit_count < least_visited_neuron.visit_count {
                least_visited_neuron = neuron;
                least_visited_neuron_key = neuron_key;
            }
        }

        return (least_visited_neuron.clone(), least_visited_neuron_key.to_string())
    }

    fn activate_neuron(mut board: Board, m: (usize, usize), brain: &Brain) -> Self {
        make_move(&mut board, m.0, m.1);
        return match brain.neurons.get(&position_to_key(&board)) {
            Some(n) => n.clone(),
            None => {
                println!("No neuron exists for this position! {}", position_to_key(&board));
                panic!()
            }
        };
    }

    fn potential(&self) -> f64 {
        self.win_count as f64 / self.visit_count as f64
    }
}

pub fn train(mut brain: Brain) -> Brain {
    // Initialize a thread pool for parallel processing

    for _simulation in 0..10000 {
        let (_, neuron_key) = Neuron::get_most_excited(&brain); // Use a selection strategy to choose a node to explore
        let outcome = practise(&mut brain, &neuron_key); // Simulate a random game from the selected node's state
        backpropagate(&mut brain, outcome); // Update node statistics based on the simulation result
        brain.neurons_used_for_crosses = vec![];
        brain.neurons_used_for_noughts = vec![];
    }

    return brain
}