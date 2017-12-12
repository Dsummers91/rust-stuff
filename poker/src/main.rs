mod card;

use card::{Card, Suit};

pub fn main() {
    let c = Card{rank:"1", suit: Suit::Spades};
    println!("{}", c);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
