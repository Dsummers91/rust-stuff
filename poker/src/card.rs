
use std::str;
use std::fmt::{Display, Formatter, Result};


#[derive(Debug)]
pub struct Card {
  pub suit: u8,
  pub rank: u8
}

impl Display for Card {
  fn fmt(self: &Card, f: &mut Formatter) -> Result {
    write!(f, "{} of {}", self.rank, self.suit)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_format_correctly() {
      let card = Card{suit:1, rank:1};
      println!("{}", card);
      assert_eq!(card, 1);
    }
}
