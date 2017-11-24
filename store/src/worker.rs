extern crate rand;

use std::str;
use std::fmt::{Display, Formatter, Result};
use self::rand::distributions::{IndependentSample, Range};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Worker {
    pub name: &'static str,
    pub age: u32,
    potential: u32, // How quickly this person can increase skills
    bagging: u32, // Skill level as a bagboy
    cashier: u32, // Skill level as a cashier
    stocking: u32, // Skill level at restocking
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

impl Worker {
    pub fn new(name: &'static str, age: u32) -> Worker {
        let mut rng = rand::thread_rng();
        let between = Range::new(0, 100);
        Worker {
            age,
            name,
            potential: between.ind_sample(&mut rng),
            bagging: between.ind_sample(&mut rng),
            cashier: between.ind_sample(&mut rng),
            stocking: between.ind_sample(&mut rng),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_worker() {
        assert_eq!(true, true);
    }
}