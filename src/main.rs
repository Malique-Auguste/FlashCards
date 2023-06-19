mod card;
mod deck;
mod tester;

use card::*;
use tester::*;
use deck::*;

fn main() {

    let mut tester: Tester = Tester::new("tester_data.json");
    tester.test(0);
    tester.save_progress();
}
