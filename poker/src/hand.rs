use card::Card;

pub trait Hand<T> {
    fn new();
}


impl Hand<Vec<Card>> for Vec<Card> {
  fn new() {
    unimplemented!();
  }
}