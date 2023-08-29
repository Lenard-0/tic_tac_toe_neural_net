use crate::game_functionality::Outcome;
use super::Brain;

impl Brain {
    pub fn backpropagate(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::CrossWins => {
                self.improve_neurons( &self.neurons_used_for_crosses.clone(), true);
                self.improve_neurons( &self.neurons_used_for_noughts.clone(), false);
            },
            Outcome::NoughtWins => {
                self.improve_neurons(&self.neurons_used_for_crosses.clone(), false);
                self.improve_neurons(&self.neurons_used_for_noughts.clone(), true);
            },
            _ => {
                self.improve_neurons(&self.neurons_used_for_crosses.clone(), false);
                self.improve_neurons(&self.neurons_used_for_noughts.clone(), false);
            }
        }
        println!("{:?}", self.neurons);
    }

    fn improve_neurons(&mut self, neurons_visited: &Vec<String>, won_game: bool) {
        let mut neurons = self.neurons.lock().unwrap();
        for neuron_key in neurons_visited {
            match neurons.get_mut(neuron_key) {
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
}