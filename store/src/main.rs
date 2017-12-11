#![feature(vec_remove_item)]

mod worker;
mod store;

use std::sync::Mutex;
use worker::Worker;
use store::Store;

fn main() {
    let store = Box::new(Store::new("test"));
    let bob = Worker::new("Bob", 18);
    println!("{}", store.name);
    store.hire_worker(Worker::new("Mary", 16));
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