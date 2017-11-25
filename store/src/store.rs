use std::sync::Mutex;
use worker::Worker;
use std::str;

pub struct Store<'a> {
    pub name: &'static str,
    pub workers: Mutex<&'a mut Vec<Worker>>,
    open_time: u32,
    close_time: u32
}

impl<'a> Store<'a> {
    pub fn new(name: &'static str, workers_vec: &'a mut Vec<Worker>) -> Store<'a> {
        let workers = Mutex::new(workers_vec);
        Store {
            name: name,
            workers: workers,
            open_time: 800,
            close_time: 1700
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store() {
        let mut workers = Box::new(Vec::new());
        let store = Store::new("test", &mut workers); 
        assert_eq!(store.name, "test");
    }

    #[test]
    fn test_store_open() {
        let mut workers = Box::new(Vec::new());
        let store = Store::new("test", &mut workers); 
        assert_eq!(store.open_time, 800);
    }
}
