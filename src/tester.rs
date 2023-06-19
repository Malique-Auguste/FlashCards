use crate::deck::Deck;
use std::{fs::{read_to_string, write}, io::{Write, self}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tester {
    path: String,
    pub inner: Vec<Deck>,
}


impl Tester {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn new(path: &str) -> Tester {
        match read_to_string(path) {
            Ok(s) => match serde_json::from_str(&s) {
                Ok(tester) => tester,
                Err(_) =>   Tester{
                    path: path.to_string(),
                    inner: Vec::new()
                }
            },
            
            Err(_) =>   Tester{
                path: path.to_string(),
                inner: Vec::new()
            }
        }
    }

    pub fn test(&mut self, deck_index: usize) {
        let mut deck: &mut Deck = &mut self.inner[deck_index];
        let mut response = String::new();

        for card in deck.inner.iter_mut() {
            print!("Card Front: {}", card.get_front());
            io::stdout().flush().expect("Unexpected error on reading input");
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
        }

    }

    pub fn save_progress(&self){
        match write(self.path.clone(), self.as_json()) {
            Ok(_) => println!("Data saved successfully."),
            Err(e) => println!("Error in saving data: '{:?}'.", e)
        }
    }

}