use crate::game_functionality::Outcome;
use super::Brain;


pub fn backpropagate(brain: &mut Brain, outcome: Outcome) {
    match outcome {
        Outcome::CrossWins => {
            improve_neurons(brain, brain.neurons_used_for_crosses.clone(), true);
            improve_neurons(brain, brain.neurons_used_for_noughts.clone(), false);
        },
        Outcome::NoughtWins => {
            improve_neurons(brain, brain.neurons_used_for_crosses.clone(), false);
            improve_neurons(brain, brain.neurons_used_for_noughts.clone(), true);
        },
        _ => {
            improve_neurons(brain, brain.neurons_used_for_crosses.clone(), false);
            improve_neurons(brain, brain.neurons_used_for_noughts.clone(), false);
        }
    }
}

fn improve_neurons(brain: &mut Brain, neurons_visited: Vec<String>, won_game: bool) {
    let mut neurons = brain.neurons.lock().unwrap();
    for neuron_key in neurons_visited {
        match neurons.get_mut(&neuron_key) {
            Some(neuron) => {
                {
                    let mut neuron = neuron.lock().unwrap();
                    neuron.visit_count += 1;
                    if won_game {
                        neuron.win_count += 1;
                    }
                } // The lock is released here when 'neuron' goes out of scope
            },
            None =>panic!("Neuron should already exist!")
        };
    }
}