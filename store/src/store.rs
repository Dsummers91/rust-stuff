use std::sync::Mutex;
use worker::Worker;
use std::str;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_store() {
        let mut workers = Box::new(Vec::new());
        let store = Box::new(Store {
            name: "test",
            workers: Mutex::new(&mut workers),
        }); 
        assert_eq!(store.name, "test");
    }
}
