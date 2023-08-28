use super::{Brain, Neuron, practise::practise, backpropagate::backpropagate};


pub fn train(mut brain: Brain) -> Brain {
    // Initialize a thread pool for parallel processing

    let simulation_attempts = 4000;
    for _simulation in 0..simulation_attempts {
        let (_, neuron_key) = Neuron::get_most_excited(&brain); // Use a selection strategy to choose a node to explore
        let outcome = practise(&mut brain, &neuron_key); // Simulate a random game from the selected node's state
        backpropagate(&mut brain, outcome); // Update node statistics based on the simulation result
        brain.neurons_used_for_crosses = vec![];
        brain.neurons_used_for_noughts = vec![];
        // brain.exploration_constant -= 1.41 / simulation_attempts as f64;
    }

    return brain
}