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
            "t" => {
                print!("{}", "The difficulty levels are:".blue());
                print!("{}", "\n1) Easy\n2) Average\n3) Hard".green());
                print!("\n\nWhich difficulty would you like to test? (enter number): ");
                io::stdout().flush().expect("Unexpected error on pushing output");
                let mut difficulty= String::new();
                io::stdin().read_line(&mut difficulty).expect("Failed to read line");
                
                let difficulty = match difficulty.trim().parse::<usize>() {
                    Ok(mut i) => {
                        i += 1;
                        if 1 < i && i < 3 {
                            Difficulty::from_num(i)
                        }
                        else {
                            println!("Err 'Input can only be 1, 2 or 3'");
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("Err in reding input '{}': '{}'", difficulty, e);
                        continue;
                    }
                };

                print!("\x1B[2J\x1B[1;1H");  


                print!("Press enter to study for a short time, type anything to study for longer: ");
                io::stdout().flush().expect("Unexpected error on pushing output");
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read line");
        
                print!("\x1B[2J\x1B[1;1H");  
                
                if response.trim() != "" {
                    tester.test(deck_index, difficulty, false)
                }
                else {
                    tester.test(deck_index, difficulty, true)
                }
            },

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
