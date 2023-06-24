use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::difficulty::Difficulty;


//Datatype representing a flashcard
#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    front: String,
    back: String,
    //For example in language learning this can be used to represent parts of speech
    additional: String,
    last_tested: SystemTime,
}

impl Card {
    pub fn new<S: Into<String>>(
        front: S,
        back: String,
        topic: String,
    ) -> Card {
        let front: String = front.into();

        Card {
            front,
            back,
            additional: topic,
            last_tested: SystemTime::now(),
        }
    }

    pub fn get_front(&self) -> &str {
        &self.front
    }

    pub fn get_back(&self) -> &str {
        &self.back
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.front)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return write!(f, "Front: {}\nBack: {:?}\nAdditional: {}\nLast Tested: {:?}", self.front, self.back, self.additional, self.last_tested);
        } else {
            return write!(
                f,
                "Front: {}, Back: {}, Additional: {}",
                self.front, self.back, self.additional
            );
        }
    }
}