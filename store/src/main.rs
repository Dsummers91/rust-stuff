#![feature(vec_remove_item)]

use std::fmt::{Display, Formatter, Result};


struct Store{
  name: &'static str,
  workers: Vec<Worker>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Worker{
  name: &'static str, 
  age: i32
}

impl Store {
  fn hire_worker(&mut self, worker: Worker) {
    self.workers.push(worker);
    println!("{} added!", worker);
  }

  fn fire_worker(&mut self, worker: Worker) {
    self.workers.remove_item(&worker);
    println!("{} removed", worker);
  }
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
  let mut store = Store{name:"test", workers:Vec::new()};
  let bob = Worker{name:"Bob", age:18};
  println!("{}", store.name);
  store.hire_worker(Worker{name:"Mary", age:16});
  store.hire_worker(bob);
  println!("{:?}", store.workers);
  println!("{}", bob);
  store.fire_worker(bob);
  println!("{:?}", store.workers);
}