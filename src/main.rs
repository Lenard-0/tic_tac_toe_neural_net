use ai_functionality::train::train;

use crate::{ai_functionality::{Brain}, game_interface::play_game};
use std::{thread, fs::{self, File}, io::Write};
use std::time::Duration;

pub mod game_functionality;
pub mod ai_functionality;
pub mod game_interface;

fn main() {
    println!("Today we are going to play a game!");
    println!("Even though my brain is made of unintelligent crystals I'll let you go first!");
    println!("The game is called noughts and crosses, but I've never played before so let me just practise a bit");

    // thread::sleep(Duration::from_secs(5));

    let genius_ai = Brain::manifest();
    println!("Ok, ready to get started!");

    loop {
        play_game(&genius_ai);
    }
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