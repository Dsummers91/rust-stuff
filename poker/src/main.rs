mod card;

use card::Card;

pub fn main() {
    let c = Card{rank:1, suit:2};
    println!("{}", c);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
