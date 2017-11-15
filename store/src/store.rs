use std::sync::Mutex;
use worker::Worker;
use std::str;
use std::fmt::{Display, Formatter, Result};

pub struct Store<'a> {
    pub name: &'static str,
    pub workers: Mutex<&'a mut Vec<Worker>>,
}


impl<'a> Store<'a> {
    pub fn hire_worker(&'a self, worker: Worker) {
        let mut workers = self.workers.lock().unwrap();
        workers.push(worker);
        println!("{} added!", worker);
    }

    pub fn fire_worker(&'a self, worker: Worker) {
        let mut workers = self.workers.lock().unwrap();
        workers.remove_item(&worker);
        println!("{} removed", worker);
    }
}