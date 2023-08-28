use std::collections::HashMap;
use std::f64::INFINITY;
use crate::game_functionality::{Board, get_possible_moves, make_move, Symbol};
use self::key::position_to_key;

pub mod key;
pub mod practise;
pub mod backpropagate;
pub mod train;

#[derive(Debug, Clone)]
pub struct Brain {
    neurons: HashMap<String, Neuron>,
    neurons_used_for_crosses: Vec<String>,
    neurons_used_for_noughts: Vec<String>,
    exploration_constant: f64
}

impl Brain {
    pub fn manifest() -> Self {
        let mut neurons = HashMap::new();
        neurons.insert("000000000".to_string(), Neuron::manifest(None));
        return Brain {
            neurons,
            neurons_used_for_crosses: vec![],
            neurons_used_for_noughts: vec![],
            exploration_constant: 1.41
        }
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
        let mut most_curious_nearon = brain.neurons.get("000000000").unwrap();
        let mut most_curious_nearon_key = "000000000";
        for (neuron_key, neuron) in &brain.neurons {
            if neuron.upper_confidence_value(brain)
            > most_curious_nearon.upper_confidence_value(brain) {
                most_curious_nearon = neuron;
                most_curious_nearon_key = neuron_key;
            }
        }

        return (most_curious_nearon.clone(), most_curious_nearon_key.to_string())
    }

    fn upper_confidence_value(&self, brain: &Brain) -> f64 {
        if self.visit_count == 0 {
            return INFINITY
        }
        // UCT(i) = Q(i) + c * sqrt(ln(N(p)) / N(i))
        let exploitation_factor = self.win_count as f64 / self.win_count as f64;
        let parent_visit_count = match &self.parent_neuron {
            Some(parent) => brain.neurons.get(parent).unwrap().visit_count as f64,
            None => 1.0
        };
        let exploration_factor =
            brain.exploration_constant * (parent_visit_count.ln() / self.visit_count as f64).sqrt();

        return exploitation_factor + exploration_factor
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