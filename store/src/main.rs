#![feature(vec_remove_item)]
mod worker;
mod store;

use std::sync::Mutex;
use std::str;
use worker::Worker;
use store::Store;


fn main() {
    let mut workers = Box::new(Vec::new());
    let store = Box::new(Store {
        name: "test",
        workers: Mutex::new(&mut workers),
    });
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
    {
        let workers = store.workers.lock().unwrap();
        println!("{:?}", *workers);
    }
    println!("{}", bob);
    store.fire_worker(bob);
    {
        let workers = store.workers.lock().unwrap();
        println!("{:?}", *workers);
    }
}

fn get_something() -> &'static str {
    "true"
}