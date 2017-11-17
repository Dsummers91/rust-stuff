use std::str;
use std::fmt::{Display, Formatter, Result};
// use attributes::Attributes;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Worker {
    pub name: &'static str,
    pub age: i32,
    potential: i32,
    bagging: i32,
    cashier: i32,
    cleaning: i32,
    stocking: i32,
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

impl Worker {
    pub fn new(name: &'static str, age: i32) -> Worker  {
        Worker {
            age,
            name,
            potential: 50,
            bagging: 50,
            cashier: 50,
            cleaning: 50,
            stocking: 50,
        }
    }
}