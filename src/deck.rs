use serde::{Deserialize, Serialize};
use std::{fs::read_to_string};

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

    pub fn get_inner_mut(&mut self, difficulty: Difficulty) -> &mut Vec<Card> {
        match difficulty {
            Difficulty::VeryEasy => &mut self.very_easy_cards,
            Difficulty::Easy => &mut self.easy_cards,
            Difficulty::Average => &mut self.average_cards,
            Difficulty::Hard => &mut self.hard_cards,
            Difficulty::VeryHard => &mut self.very_hard_cards,
        }
    }
}