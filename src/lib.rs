use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::{near_bindgen};

mod data_input;



//const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String
}

