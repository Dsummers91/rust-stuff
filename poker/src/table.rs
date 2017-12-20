use player::{Player};

pub struct Table {
  seats: u8,
  buyin: u32, // For Tournaments it is entry fee, cash games are buyin (2NL is .01/.02, 100 big blinds) 
}