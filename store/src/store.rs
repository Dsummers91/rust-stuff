use std::sync::Mutex;
use worker::Worker;
use std::str;

pub struct Store {
    pub name: &'static str,
    pub workers: Mutex<Vec<Worker>>,
    open_time: u32,
    close_time: u32
}

impl Store {
    pub fn new(name: &'static str) -> Store {
        let workers = Mutex::new(Vec::new());
        Store {
            name: name,
            workers: workers,
            open_time: 800,
            close_time: 1700
        }
    }
}

impl Store {
    pub fn hire_worker(&self, worker: Worker) {
        let mut workers = self.workers.lock().unwrap();
        workers.push(worker);
        println!("{} added!", worker);
    }

    pub fn fire_worker(&self, worker: Worker) {
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
        let store = Store::new("test"); 
        assert_eq!(store.name, "test");
    }

    #[test]
    fn test_store_open() {
        let store = Store::new("test"); 
        assert_eq!(store.open_time, 800);
    }
}
