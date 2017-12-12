
use card::{Card, Suit};
use rand::{thread_rng, Rng};


pub struct Deck<'a> {
  pub deck: &'a mut Vec<Card>
}

impl<'a> Deck<'a> {
  // pub fn new() -> Deck {
   
  // }

  pub fn init() {
      println!("{}", 2)
    // for suit in Suit {
    //   println!("{}", suit)
    // }
  }

  fn shuffle(self: &Deck) {
    let mut deck = self.clone();
    let mut rng = thread_rng();
    
    rng.shuffle(&mut deck.deck)
  }
}

#[cfg(test)]
  mod tests {
  use super::*;
    #[test]
    fn should_be_true() {
      let deck = Deck::init();
      assert_eq!(deck.)
    }
}
