#![feature(vec_remove_item)]
mod worker;

use std::sync::{Mutex};
use std::str;
use std::fmt::{Display, Formatter, Result};
use worker::Worker;

struct Store<'a> {
    name: &'static str,
    workers: Mutex<&'a mut Vec<Worker>>,
}


impl<'a> Store<'a> {
    fn hire_worker(&'a self, worker: Worker) {
        let mut workers = self.workers.lock().unwrap();
        workers.push(worker);
        println!("{} added!", worker);
    }

    fn fire_worker(&'a self, worker: Worker) {
        let mut workers = self.workers.lock().unwrap();
        workers.remove_item(&worker);
        println!("{} removed", worker);
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

fn main() {
    let mut workers = Vec::new();
    let store = Store {
        name: "test",
        workers: Mutex::new(&mut workers),
    };
    let bob = Worker {
        name: "Bob",
        age: 18,
    };
    let g = get_something();
    println!("{}", g);
    println!("{}", store.name);
    store.hire_worker(Worker {
        name: "Mary",
        age: 16,
    });
    store.hire_worker(bob);

    println!("{:?}", store.workers);
    println!("{}", bob);
    store.fire_worker(bob);
    store.fire_worker(bob);
    println!("{:?}", store.workers);
}

fn get_something() -> &'static str {
  "true"
}