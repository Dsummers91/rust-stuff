use card::{Card};
use deck::{Deck};

pub trait Player<T> {
    fn new(&mut self);
}


impl Player<Vec<Card>> for Vec<Card> {
  fn new(self: &mut Self) {
    unimplemented!();
  }
}

#[cfg(test)]
  mod tests {
  use super::*;
    #[test]
    fn should_be_true() {
    assert_eq!(2 + 2, 4);
    }
}

