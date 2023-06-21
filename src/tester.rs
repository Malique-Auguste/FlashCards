use crate::deck::Deck;
use crate::card::Card;
use std::{fs::{read_to_string, read_dir, write}, io::{Write, self}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tester {
    folder_path: String,
    pub inner: Vec<Deck>,
}


impl Tester {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn get_generic(folder_path: &str) -> Tester {
        Tester{
            folder_path: folder_path.to_string(),
            inner: Vec::new()
        }
    }

    pub fn new(folder_path: &str) -> Tester {
        let mut tester = Tester::get_generic(folder_path);
        
        let file_paths = match read_dir(folder_path) {
            Ok(p) => p,
            Err(e) => {
                println!("err 1: {}", e);

                return tester
            }
        };

        println!("{:?}\n", file_paths);

        for path in file_paths {
            match read_to_string(path.unwrap().path()) {
                Ok(s) => tester.inner.push( match serde_json::from_str(&s) {
                    Ok(deck) => deck,
                    Err(e) => {
                        println!("errs 2: {}", e);
                        continue
                    }
                }),
                Err(e) => {
                    println!("err 3: {}", e);

                    continue
                }
            }
        }

        tester
            
        
    }

    pub fn extend_deck(&mut self, deck_index: usize) {
        if self.inner.len() == 0 {
            println!("Deck is empty.");
            return
        }

        let mut deck: &mut Deck = &mut self.inner[deck_index];

        loop {
            print!("Enter front of card: ");
            io::stdout().flush().expect("Unexpected error on reading input");
            let mut front = String::new();
            io::stdin().read_line(&mut front).expect("Failed to read line");

            print!("Enter back of card: ");
            io::stdout().flush().expect("Unexpected error on reading input");
            let mut back = String::new();
            io::stdin().read_line(&mut back).expect("Failed to read line");

            print!("Enter any additional: ");
            io::stdout().flush().expect("Unexpected error on reading input");
            let mut additional = String::new();
            io::stdin().read_line(&mut additional).expect("Failed to read line");

            deck.inner.push(Card::new(front.trim().to_string(), back.trim().to_string(), additional.trim().to_string()));

            print!("Press enter to continue, type anything to exit: ");
            io::stdout().flush().expect("Unexpected error on reading input");
            additional = String::new();
            io::stdin().read_line(&mut additional).expect("Failed to read line");

            println!();

            if additional.trim() != "" {
                return
            }

        }
    }

    pub fn test(&mut self, deck_index: usize) {
        if self.inner.len() == 0 {
            println!("No decks available.");
            return
        }

        let mut deck: &mut Deck = &mut self.inner[deck_index];
        let mut response = String::new();

        for card in deck.inner.iter_mut() {
            print!("Card Front: {}", card.get_front());
            io::stdout().flush().expect("Unexpected error on pushing output");
            io::stdin().read_line(&mut response).expect("Failed to read line");

            response = String::new();
            print!("Card Back: {}\nCorrect(c) or wrong(w): ", card.get_back());
            io::stdout().flush().expect("Unexpected error on reading input");
            io::stdin().read_line(&mut response).expect("Failed to read line");

            match response.trim() {
                "w" => card.increase_difficulty(),
                "c" => card.decrease_difficulty(),
                _ => panic!("Expected 'c' or 'w', got: '{:#?}'", response),
            }

            print!("\x1B[2J\x1B[1;1H");
        }

    }

    pub fn save_progress(&self){
        for i in  0..self.inner.len() { 
            match write(format!("{}/{}{}.json", self.folder_path, "deck_", i), self.inner[i].as_json()) {
                Ok(_) => println!("Data saved successfully."),
                Err(e) => println!("Error in saving data: '{:?}'.", e)
            }
        }
    }

}