use ai_functionality::train::train;

use crate::{ai_functionality::{Brain, disk_interface::write_existing_neurons}, game_interface::play_game};
use std::{thread, fs::{self, File}, io::{Write, self}};
use std::time::Duration;

pub mod game_functionality;
pub mod ai_functionality;
pub mod game_interface;

fn main() {
    println!("Today we are going to play a game!");
    println!("Even though my brain is made of unintelligent crystals I'll let you go first!");
    println!("The game is called noughts and crosses, but I've never played before so let me just practise a bit");

    // thread::sleep(Duration::from_secs(5));

    let mut genius_ai = Brain::manifest();
    println!("Ok, ready to get started!");

    loop {
        play_game(&mut genius_ai);
        println!("Would you like to continue playing (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "y".to_string() {
            continue;
        } else {
            break;
        }
    }
    write_existing_neurons(&genius_ai);
}

#[test]
fn train_ai() {
    train(Brain::manifest());
}

// #[test]
// fn create_first_neuron() {
//     let path = format!("src/neurons/{}.txt", "000000000");
//     let mut file = File::create(path).unwrap();
//     file.write_all(format!("000000000|0|0|0").as_bytes()).unwrap();
// }