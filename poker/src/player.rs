use card::{Card};
use deck::{Deck};

pub trait Hand<T> {
    fn new(&mut self, Vec<Card>);
}


impl Hand<Vec<Card>> for Vec<Card> {
  fn new(self: &mut Self, deck: Vec<Card>) {
    unimplemented!();
  }
}

#[cfg(test)]
  mod tests {
  // use super::*;
    #[test]
    fn should_be_true() {
    assert_eq!(2 + 2, 4);
    }
}

