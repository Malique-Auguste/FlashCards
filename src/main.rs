mod card;
mod deck;
mod tester;

use tester::Tester;
use deck::Deck;
use std::io;
use std::io::Write;
use colored::Colorize;


fn main() {

    let mut tester: Tester = Tester::new("tester_data");
    let mut deck_names = String::new();

    for deck in tester.inner.iter() {
        deck_names.push_str(&deck.name);
        deck_names.push('\n');
    }

    loop {
        
        let mut colored_text = "List of Decks:".blue();
        print!("{}", colored_text);

        colored_text = deck_names.green();
        print!("\n{}", colored_text);
        print!("\nWhich deck would you like to use?: ");
        io::stdout().flush().expect("Unexpected error on pushing output");
        let mut deck_name = String::new();
        io::stdin().read_line(&mut deck_name).expect("Failed to read line");

        print!("\x1B[2J\x1B[1;1H");

        let deck_index = match tester.inner.iter().position(|deck| deck.name == deck_name.trim()) {
            Some(i) => i,
            None => {
                println!("'{}' is nto a possible deck.\n", deck_name);
                continue;
            }
        };

        print!("Would you like to do a test(t) or extend the deck(e)?: ");
        io::stdout().flush().expect("Unexpected error on pushing output");
        let mut activity = String::new();
        io::stdin().read_line(&mut activity).expect("Failed to read line");    

        print!("\x1B[2J\x1B[1;1H");  

        match activity.trim() {
            "t" => tester.test(deck_index),
            "e" => tester.extend_deck(deck_index),
            _ => {
                println!("'{}' doesn't correspond to a given actvity.\n", activity);
                continue;
            },
        }

        tester.save_progress()
    }

}
