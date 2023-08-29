use std::sync::{Arc, MutexGuard};
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, sync::Mutex};
use std::f64::INFINITY;
use crate::game_functionality::{Board, get_possible_moves, make_move, Symbol};
use self::disk_interface::get_existing_neurons;
use self::key::position_to_key;
use std::fs;

pub mod key;
pub mod practise;
pub mod backpropagate;
pub mod train;
pub mod disk_interface;

#[derive(Debug, Clone)]
pub struct Brain {
    pub neurons: Arc<Mutex<HashMap<String, Arc<Mutex<Neuron>>>>>,
    neurons_used_for_crosses: Vec<String>,
    pub neurons_used_for_noughts: Vec<String>,
    exploration_constant: f64
}

impl Brain {
    pub fn manifest() -> Self {
        return Brain {
            neurons: Arc::new(Mutex::new(get_existing_neurons())),
            neurons_used_for_crosses: vec![],
            neurons_used_for_noughts: vec![],
            exploration_constant: 3.0
        }
    }

    pub fn choose_best_move(&mut self, board: &mut Board) {
        let possible_moves = get_possible_moves(board);
        let mut best_neuron = Neuron::activate_neuron(board.clone(), possible_moves[0], self);
        let mut best_move = possible_moves[0];
        let mut first_move = true;
        for m in possible_moves {
            let current_neuron = Neuron::activate_neuron(board.clone(), m, self);
            if !first_move {
                let current_neuron_better = {
                    let current_neuron_lock = current_neuron.lock().unwrap();
                    let best_neuron_lock = best_neuron.lock().unwrap();
                    current_neuron_lock.win_count as f64 / current_neuron_lock.visit_count as f64
                    > best_neuron_lock.win_count as f64 / best_neuron_lock.visit_count as f64
                };
                if current_neuron_better {
                    best_neuron = current_neuron.clone();
                    best_move = m;
                }
            }
            first_move = false;
        }

        board[best_move.0][best_move.1] = Some(Symbol::Nought);
        self.neurons_used_for_noughts.push(position_to_key(board));
    }
}

#[derive(Debug, Clone)]
pub struct Neuron {
    pub visit_count: usize,
    pub win_count: usize,
    parent_neuron: Option<String>,
    // children_neurons: Vec<Arc<Mutex<Neuron>>>
}

impl Neuron {
    pub fn manifest(parent_neuron: Option<String>) -> Self {
        return Neuron {
            visit_count: 0,
            win_count: 0,
            parent_neuron
        }
    }

    pub fn generate(visit_count: usize, win_count: usize, parent_neuron: Option<String>) -> Neuron {
        return Neuron { visit_count, win_count, parent_neuron }
    }

    pub fn get_most_excited(brain: &Brain) -> (Arc<Mutex<Self>>, String) {
        let neurons = brain.neurons.lock().unwrap();
        let mut most_curious_nearon = neurons.get("000000000").unwrap().clone();
        let mut most_curious_nearon_key = "000000000";
        for (neuron_key, neuron) in neurons.iter() {
            if neuron_key != most_curious_nearon_key {
                // {
                //     println!("n: {}", neuron.lock().unwrap().upper_confidence_value(brain, &neurons));
                //     println!("cn: {}", most_curious_nearon.lock().unwrap().upper_confidence_value(brain, &neurons));
                //     thread::sleep(Duration::from_secs(2));
                // }
                let current_upper_confidence = neuron.lock().unwrap().upper_confidence_value(brain, &neurons);
                if current_upper_confidence == INFINITY {
                    return (neuron.clone(), neuron_key.to_string())
                }
                if current_upper_confidence
                > most_curious_nearon.lock().unwrap().upper_confidence_value(brain, &neurons) {
                    most_curious_nearon = neuron.clone();
                    most_curious_nearon_key = &neuron_key;
                }
            }
        }

        return (most_curious_nearon.clone(), most_curious_nearon_key.to_string())
    }

    fn upper_confidence_value(&self, brain: &Brain, neurons: &MutexGuard<'_, HashMap<String, Arc<Mutex<Neuron>>>>) -> f64 {
        if self.visit_count == 0 {
            return INFINITY
        }

        // UCT(i) = Q(i) + c * sqrt(ln(N(p)) / N(i))
        let exploitation_factor = self.win_count as f64 / self.visit_count as f64;
        // let parent_visit_count = match &self.parent_neuron {
        //     Some(parent) =>  neurons.get(parent).unwrap().lock().unwrap().visit_count as f64,
        //     None => 1.0
        // };
        let exploration_factor =
            brain.exploration_constant * (1.0 / self.visit_count as f64).sqrt();
            // brain.exploration_constant * (parent_visit_count.ln() / self.visit_count as f64).sqrt();

        return exploitation_factor + exploration_factor
    }

    fn activate_neuron(mut board: Board, m: (usize, usize), brain: &Brain) -> Arc<Mutex<Self>> {
        make_move(&mut board, m.0, m.1);
        return match brain.neurons.lock().unwrap().get(&position_to_key(&board)) {
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