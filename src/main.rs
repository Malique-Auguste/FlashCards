mod card;
mod deck;
mod tester;
mod difficulty;

use tester::Tester;
use deck::Deck;
use difficulty::Difficulty;

use std::io;
use std::io::Write;
use std::ops::Add;
use colored::Colorize;


fn main() {

    let mut tester: Tester = Tester::new("tester_data");
    let mut deck_names = String::new();

    for i in 0..tester.inner.len() {
        let line = format!("{}) {}\n", i + 1, tester.inner[i].name);
        deck_names.push_str(line.as_str())
    }

    loop {
        
        let mut colored_text = "List of Decks:".blue();
        print!("{}", colored_text);

        colored_text = deck_names.green();
        print!("\n{}", colored_text);
        print!("\nWhich deck would you like to use? (enter number): ");
        io::stdout().flush().expect("Unexpected error on pushing output");
        let mut deck_index = String::new();
        io::stdin().read_line(&mut deck_index).expect("Failed to read line");
        
        let deck_index = match deck_index.trim().parse::<usize>() {
            Ok(i) => {
                if i - 1 < tester.inner.len() {
                    i - 1
                }
                else {
                    println!("Given index '{}' is out of range.", deck_index);
                    continue;
                }
            },
            Err(e) => {
                println!("Err in reding deck '{}': '{}'", deck_index, e);
                continue;
            }
        };

        print!("\x1B[2J\x1B[1;1H");      

        print!("Would you like to do a test(t) or extend the deck(e)?: ");
        io::stdout().flush().expect("Unexpected error on pushing output");
        let mut activity = String::new();
        io::stdin().read_line(&mut activity).expect("Failed to read line");    

        print!("\x1B[2J\x1B[1;1H");  

        match activity.trim() {
            "t" => tester.test(deck_index, Difficulty::VeryEasy, true),
            "e" => tester.extend_deck(deck_index),
            _ => {
                println!("'{}' doesn't correspond to a given actvity.\n", activity);
                continue;
            },
        }

        tester.save_progress();

        break;
    }

}
