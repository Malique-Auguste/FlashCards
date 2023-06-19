use serde::{Deserialize, Serialize};
use std::time::SystemTime;


#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Average,
    Hard,
    VeryHard,
}

impl Difficulty {
    fn as_num(&self) -> u8 {
        match self {
            Difficulty::VeryEasy => 1,
            Difficulty::Easy => 2,
            Difficulty::Average => 3,
            Difficulty::Hard => 4,
            Difficulty::VeryHard => 5,
        }
    }

    fn change(&mut self, increase: bool) {
        let mut num = self.as_num();

        match increase {
            true => num += 1,
            false => num -= 1
        }

        *self = match num {
            0 => Difficulty::VeryEasy,
            1 => Difficulty::VeryEasy,
            2 => Difficulty::Easy,
            3 => Difficulty::Average,
            4 => Difficulty::Hard,
            _ => Difficulty::VeryHard
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    front: String,
    back: String,
    additional: String,
    last_tested: SystemTime,
    difficulty: Difficulty,
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
            difficulty: Difficulty::Average,
        }
    }

    pub fn get_front(&self) -> &str {
        &self.front
    }

    pub fn get_back(&self) -> &str {
        &self.back
    }

    pub fn increase_difficulty(&mut self) {
        self.difficulty.change(true);
        self.last_tested = SystemTime::now();
    }

    pub fn decrease_difficulty(&mut self) {
        self.difficulty.change(false);
        self.last_tested = SystemTime::now();
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
            return write!(f, "Front: {}\nBack: {:?}\nAdditional: {}\nLast Tested: {:?}\nDifficulty: {:?}", self.front, self.back, self.additional, self.last_tested, self.difficulty);
        } else {
            return write!(
                f,
                "Front: {}, Back: {}, Additional: {}",
                self.front, self.back, self.additional
            );
        }
    }
}