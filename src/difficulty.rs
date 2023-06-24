use serde::{Deserialize, Serialize};

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

    //Increases or decrease "difficulty" of self
    pub fn change(&mut self, increase: bool) {
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