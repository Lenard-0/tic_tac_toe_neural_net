use std::thread;

use super::{Brain, Neuron, practise::practise};


pub fn train(brain: Brain) -> Brain {

    let mut threads = vec![];
    for _ in 0..20 {
        let mut brain_clone = brain.clone();
        let thread = thread::spawn(move || {
            let simulation_attempts = 1000;
            for _simulation in 0..simulation_attempts {
                let (_, neuron_key) = Neuron::get_most_excited(&brain_clone); // Use a selection strategy to choose a node to explore
                let outcome = practise(&mut brain_clone, &neuron_key); // Simulate a random game from the selected node's state
                brain_clone.backpropagate(outcome); // Update node statistics based on the simulation result
                brain_clone.neurons_used_for_crosses = vec![];
                brain_clone.neurons_used_for_noughts = vec![];
                // brain.exploration_constant -= 1.41 / simulation_attempts as f64;
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }



    return brain
}