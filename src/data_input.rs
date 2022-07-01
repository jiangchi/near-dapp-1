use crate::*;
use std::cmp::Ordering;

use near_sdk::{near_bindgen,env};
#[near_bindgen]
impl Contract {
    pub fn get_puzzle_number(&self) -> String {
        self.crossword_solution.to_string()
    }

    pub fn set_solution(&mut self, solution: String)  {
        self.crossword_solution = solution;
    }

    /// .
    pub fn guess_solution(&mut self, solution: String) -> &str {
        if solution == self.crossword_solution {

            env::log_str("You guessed right!");
           // self.crossword_solution.to_string() +r#"You guessed right!"#
        } else {
            env::log_str("Try again.");
           // self.crossword_solution.to_string() +r#"Try again."#
        }
        match self.crossword_solution.cmp(&solution) {
            Ordering::Less => r#"You guessed Too big!!"#,
            Ordering::Greater => r#"You guessed Too small!!"#,
            Ordering::Equal =>r#"You guessed is right!!"#,

    }
    
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from cargo.toml's 'name' key
 */
#[cfg(not(target = "wasm32"))]
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use near_sdk::{VMContext, test_utils::VMContextBuilder, testing_env};

    use super::*;
    
    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }
    #[test] 
    pub fn test_guess_solution() {
       
       //let context = get_context(false);
        let  contract = Contract {crossword_solution: "66".to_string()};
       let x= Contract::get_puzzle_number(&contract);
       
        assert_eq!("66",x,"与设置的数据不相符！");
      

    }
    // TESTS HERE
}