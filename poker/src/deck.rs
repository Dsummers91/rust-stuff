use card::{Card, Suit};
use rand::{thread_rng, Rng};

pub trait Deck<T> {
    fn new() -> Self;
    fn populate(mut self: &mut Self);
    fn shuffle(mut self: &mut Self);
}

impl Deck<Vec<Card>> for Vec<Card> {
    // pub fn new() -> Deck {

    // }

    fn new() -> Vec<Card> {
        println!("{}", 2);
        let mut deck: Vec<Card> = Vec::new();
        deck.populate();
        deck.shuffle();
        deck
    }

    fn populate(mut self: &mut Self) {
      for suit in Suit::iter() {
        for rank in 2..14 {
          let r: &str = &rank.to_string();
          self.push(Card::new(rank, Suit::Spades));
        }
      }
    }

    fn shuffle(mut self: &mut Self)  {
        let mut rng = thread_rng();
        rng.shuffle(&mut self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_be_true() {
        let mut f: Vec<Card> = Deck::new();
        f.shuffle();
    }
}
