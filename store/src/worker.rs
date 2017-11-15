use std::str;
use std::fmt::{Display, Formatter, Result};
// use attributes::Attributes;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Worker {
    pub name: &'static str,
    pub age: i32,
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}