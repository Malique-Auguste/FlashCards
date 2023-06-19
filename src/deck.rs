use serde::{Deserialize, Serialize};
use std::{fs::read_to_string};

use crate::card::{Card};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    pub inner: Vec<Card>,
}

impl Deck {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn new(name: String) -> Deck {
        Deck {
            name: name,
            inner: Vec::new(),
        }  
    }
}