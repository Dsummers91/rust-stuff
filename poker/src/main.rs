extern crate rand;

mod card;
mod hand;
mod deck;


use card::{Card, Suit};
use deck::{Deck};


pub fn main() {
    let c = Card{rank:"1", suit: Suit::Spades};
    println!("{}", c);
    // let d: <Vec<Card>> = Deck::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
