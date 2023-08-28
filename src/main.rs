use crate::{ai_functionality::{Brain, train}, game_interface::play_game};
use std::thread;
use std::time::Duration;

pub mod game_functionality;
pub mod ai_functionality;
pub mod game_interface;

fn main() {
    println!("Today we are going to play a game!");
    println!("Even though my brain is made of unintelligent crystals I'll let you go first!");
    println!("The game is called noughts and crosses, but I've never played before so let me just practise a bit");

    thread::sleep(Duration::from_secs(5));

    let genius_ai = train(Brain::manifest());
    println!("{:#?}", genius_ai);
    println!("Ok, ready to get started!");

    play_game(genius_ai);
}
