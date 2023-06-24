use crate::{deck::Deck};
use crate::difficulty::{Difficulty, self};
use crate::card::Card;
use std::{fs::{read_to_string, read_dir, write}, io::{Write, self}};
use serde::{Deserialize, Serialize};

use rand::prelude::*;

pub fn shuffle<I>(to_shuffle: &mut Vec<I>) {
    let mut rng = rand::thread_rng();
    let length = to_shuffle.len();
    let mut current_pos = 0;

    while current_pos < to_shuffle.len() {
        let temp_pos = rng.gen_range(0..length - 1);

        let current = to_shuffle.remove(current_pos);
        let temp = to_shuffle.remove(temp_pos);

        to_shuffle.insert(temp_pos, current);
        to_shuffle.insert(current_pos, temp);

        current_pos += 2;
    }
}

//Represents the DataType that "manages" the program. It sores all the deck of cards and is used to test the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tester {
    folder_path: String,
    pub inner: Vec<Deck>,
}


impl Tester {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    //Creates an empty Tester
    fn get_generic(folder_path: &str) -> Tester {
        Tester{
            folder_path: folder_path.to_string(),
            inner: Vec::new()
        }
    }

    pub fn new(folder_path: &str) -> Tester {
        let mut tester = Tester::get_generic(folder_path);
        
        //gets the paths to all the files in the given folder. However if there is an error it returns the generic tester.
        let file_paths = match read_dir(folder_path) {
            Ok(p) => p,
            Err(e) => {
                println!("err 1: {}", e);

                return tester
            }
        };

        println!("{:?}\n", file_paths);

        //reads each deck, however if there is an error in reading it, that deck is skipped
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

            deck.add_card(Card::new(front.trim().to_string(), back.trim().to_string(), additional.trim().to_string()), Difficulty::Average);

            print!("Press enter to continue, type anything to exit: ");
            io::stdout().flush().expect("Unexpected error on reading input");
            additional = String::new();
            io::stdin().read_line(&mut additional).expect("Failed to read line");

            println!();

            if additional.trim() != "" {
                return
            }

            print!("\x1B[2J\x1B[1;1H");  


        }
    }

    pub fn test(&mut self, deck_index: usize, mut difficulty: Difficulty, short: bool) {
        if self.inner.len() == 0 {
            println!("No decks available.");
            return
        }

        let mut deck: &mut Deck = &mut self.inner[deck_index];
        let mut cards_to_test: &mut Vec<Card> = &mut Vec::new();
        let mut cards_to_test_length: usize;
        let mut changes = Vec::new();

        if short {
            match difficulty {
                Difficulty::VeryEasy => {
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();

                    if cards_to_test_length <= 10 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes = Tester::test_cards(&mut cards_to_test[0..10]);
                    }


                    Tester::change_difficulty(deck, difficulty, changes);

                    difficulty.change(true);
                    
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 5 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes = Tester::test_cards(&mut cards_to_test[0..5]);
                    }

                    Tester::change_difficulty(deck, difficulty, changes);

                }

                Difficulty::Easy | Difficulty::Average | Difficulty::Hard => {
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 10 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes= Tester::test_cards(&mut cards_to_test[0..10]);
                    }

                    difficulty.change(true);
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 5 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes = Tester::test_cards(&mut cards_to_test[0..5]);
                    }

                    Tester::change_difficulty(deck, difficulty, changes);


                    difficulty.change(false);
                    difficulty.change(false);
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 5 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes = Tester::test_cards(&mut cards_to_test[0..5]);
                    }

                    Tester::change_difficulty(deck, difficulty, changes);

                }

                Difficulty::VeryHard => {
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 10 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes= Tester::test_cards(&mut cards_to_test[0..10]);
                    }

                    difficulty.change(false);
                    cards_to_test = deck.get_inner_mut(difficulty);
                    cards_to_test_length = cards_to_test.len();
                    if cards_to_test_length <= 5 {
                        changes = Tester::test_cards(&mut cards_to_test[0..cards_to_test_length]);
                    }
                    else {
                        changes = Tester::test_cards(&mut cards_to_test[0..5]);
                    }

                    Tester::change_difficulty(deck, difficulty, changes);

                }
            }
        }

        else { 
                Tester::test_cards(&mut deck.get_inner_mut(difficulty));
            }        

    }

    fn test_cards(cards: &mut [Card]) -> Vec<bool> {
        let mut response = String::new();
        let mut card_changes = Vec::new();

        for card in cards {
            print!("Card Front: {}", card.get_front());
            io::stdout().flush().expect("Unexpected error on pushing output");
            io::stdin().read_line(&mut response).expect("Failed to read line");

            response = String::new();
            print!("Card Back: {}\nCorrect(c) or wrong(w): ", card.get_back());
            io::stdout().flush().expect("Unexpected error on reading input");
            io::stdin().read_line(&mut response).expect("Failed to read line");

            match response.trim() {
                "w" => card_changes.push(true),
                "c" => card_changes.push(false),
                _ => panic!("Expected 'c' or 'w', got: '{:#?}'", response),
            }

            print!("\x1B[2J\x1B[1;1H"); 
        }

        card_changes
    }

    fn change_difficulty(deck: &mut Deck, mut current_dicciculty: Difficulty, changes: Vec<bool>) {
        let mut easier_cards = Vec::new();
        let mut harder_cards = Vec::new();

        for (card, change) in deck.get_inner_mut(current_dicciculty).drain(0..changes.len()).zip(changes) {
            if change == true {
                harder_cards.push(card)
            }
            else {
                easier_cards.push(card)
            }
        }

        current_dicciculty.change(true);
        deck.get_inner_mut(current_dicciculty).append(&mut harder_cards);

        current_dicciculty.change(false);
        current_dicciculty.change(false);
        deck.get_inner_mut(current_dicciculty).append(&mut easier_cards);
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