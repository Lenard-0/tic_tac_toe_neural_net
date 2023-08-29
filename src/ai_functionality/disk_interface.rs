use std::{collections::HashMap, sync::{Arc, Mutex}, fs::{self, File}, path::Path, io::Write, thread, time::Duration};

use super::{Neuron, Brain};


#[derive(Debug, PartialEq)]
enum ReadStage {
    Key,
    WinCount,
    VisitCount,
    ParentKey
}

pub fn get_existing_neurons() -> HashMap<String, Arc<Mutex<Neuron>>> {
    let mut neurons: HashMap<String, Arc<Mutex<Neuron>>> = HashMap::new();
        for entry in fs::read_dir("src/neurons").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                _ => {
                    // println!("corrupt: {:?}", err);
                    continue;
                },
            };
            let mut win_count_str = "".to_string();
            let mut visit_count_str = "".to_string();
            let mut key = "".to_string();
            let mut parent_key = "".to_string();
            let mut read_stage = ReadStage::Key;
            for ch in contents.chars() {
                if read_stage == ReadStage::Key && ch == '|' {
                    read_stage = ReadStage::WinCount;
                    continue;
                } else if read_stage == ReadStage::WinCount && ch == '|' {
                    read_stage = ReadStage::VisitCount;
                    continue;
                } else if read_stage == ReadStage::VisitCount && ch == '|' {
                    read_stage = ReadStage::ParentKey;
                    continue;
                } else if read_stage == ReadStage::ParentKey && ch == '|' {
                    break;
                }
                match read_stage {
                    ReadStage::Key => key.push(ch),
                    ReadStage::WinCount => win_count_str.push(ch),
                    ReadStage::VisitCount => visit_count_str.push(ch),
                    ReadStage::ParentKey => parent_key.push(ch),
                }
            }
            let neuron = Neuron::generate(
                visit_count_str.parse().expect("Failed to parse string into number"),
                win_count_str.parse().expect("Failed to parse string into number"),
                parse_parent_key(parent_key)
            );
            neurons.insert(key, Arc::new(Mutex::new(neuron)));
        }

        return neurons
}

fn parse_parent_key(parent_key: String) -> Option<String> {
    if parent_key == "0".to_string() {
        return None
    } else {
        return Some(parent_key)
    }
}

pub fn write_existing_neurons(brain: &Brain) {
    let neurons = brain.neurons.lock().unwrap();
    for (key, neuron) in neurons.iter() {
        let neuron = neuron.lock().unwrap();
        let path = format!("src/neurons/{}.txt", key);
        let mut file = File::create(path).unwrap();
        match file.write_all(format!("{}|{}|{}|{}", key, neuron.win_count, neuron.visit_count, convert_parent_key(&neuron.parent_neuron)).as_bytes()) {
            Ok(_) => {},
            _ => { println!("key failed to write at: {}", key); }
        };
    }
}

fn convert_parent_key(parent_key: &Option<String>) -> String {
    match parent_key {
        Some(key) => key.to_string(),
        None => "0".to_string()
    }
}