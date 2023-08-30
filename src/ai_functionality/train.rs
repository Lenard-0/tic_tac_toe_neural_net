use std::thread;
use std::time::Duration;
use super::{Brain, Neuron, practise::practise, disk_interface::write_existing_neurons};

pub fn train(brain: Brain) -> Brain {
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    for _ in 0..=1000 {
        let mut brain_clone = brain.clone();
        let thread = thread::spawn(move || {
            let simulation_attempts = 5;
            for _simulation in 0..simulation_attempts {
                let (_, neuron_key) = Neuron::get_most_excited(&brain_clone, true); // Use a selection strategy to choose a node to explore
                // println!("neuron_key: {}", neuron_key);
                // thread::sleep(Duration::from_secs(2));
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

    write_existing_neurons(&brain);

    return brain
}