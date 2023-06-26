use serde::{Deserialize, Serialize};
use std::{fs::read_to_string};
use rand::prelude::*;

use crate::card::{Card};
use crate::difficulty::Difficulty;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    very_easy_cards: Vec<Card>,
    easy_cards: Vec<Card>,
    average_cards: Vec<Card>,
    hard_cards: Vec<Card>,
    very_hard_cards: Vec<Card>,
}

impl Deck {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn new(name: String) -> Deck {
        Deck {
            name: name,
            very_easy_cards: Vec::new(),
            easy_cards: Vec::new(),
            average_cards: Vec::new(),
            hard_cards: Vec::new(),
            very_hard_cards: Vec::new(),
        }  
    }

    pub fn add_card(&mut self, card: Card, difficulty: Difficulty) {
        match difficulty {
            Difficulty::VeryEasy => self.very_easy_cards.push(card),
            Difficulty::Easy => self.easy_cards.push(card),
            Difficulty::Average => self.average_cards.push(card),
            Difficulty::Hard => self.hard_cards.push(card),
            Difficulty::VeryHard => self.very_hard_cards.push(card),
        }
    }

    pub fn get_inner_mut(&mut self, difficulty: Difficulty, shuffle: bool) -> &mut Vec<Card> {
        if shuffle {
            let mut cards = match difficulty {
                Difficulty::VeryEasy => &mut self.very_easy_cards,
                Difficulty::Easy => &mut self.easy_cards,
                Difficulty::Average => &mut self.average_cards,
                Difficulty::Hard => &mut self.hard_cards,
                Difficulty::VeryHard => &mut self.very_hard_cards,
            };

            Deck::shuffle(cards);

            cards
        }
        else {
            match difficulty {
                Difficulty::VeryEasy => &mut self.very_easy_cards,
                Difficulty::Easy => &mut self.easy_cards,
                Difficulty::Average => &mut self.average_cards,
                Difficulty::Hard => &mut self.hard_cards,
                Difficulty::VeryHard => &mut self.very_hard_cards,
            }
        }
    }

    fn shuffle<I>(to_shuffle: &mut Vec<I>) {
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
}