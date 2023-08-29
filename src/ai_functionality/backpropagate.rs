use crate::game_functionality::Outcome;
use super::Brain;


enum Result {
    Win,
    Loss,
    Draw
}
impl Brain {
    pub fn backpropagate(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::CrossWins => {
                self.improve_neurons( &self.neurons_used_for_crosses.clone(), Result::Win);
                self.improve_neurons( &self.neurons_used_for_noughts.clone(), Result::Loss);
            },
            Outcome::NoughtWins => {
                self.improve_neurons(&self.neurons_used_for_crosses.clone(), Result::Loss);
                self.improve_neurons(&self.neurons_used_for_noughts.clone(), Result::Win);
            },
            _ => {
                self.improve_neurons(&self.neurons_used_for_crosses.clone(), Result::Draw);
                self.improve_neurons(&self.neurons_used_for_noughts.clone(), Result::Draw);
            }
        }
    }

    fn improve_neurons(&mut self, neurons_visited: &Vec<String>, result: Result) {
        let mut neurons = self.neurons.lock().unwrap();
        for neuron_key in neurons_visited {
            match neurons.get_mut(neuron_key) {
                Some(neuron) => {
                    {
                        let mut neuron = neuron.lock().unwrap();
                        neuron.visit_count += 1;
                        match result {
                            Result::Win => neuron.win_count += 3,
                            Result::Draw => neuron.win_count += 1,
                            Result::Loss => {}
                        }
                    } // The lock is released here when 'neuron' goes out of scope
                },
                None => panic!("Neuron should already exist!")
            };
        }
    }
}