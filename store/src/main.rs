#![feature(vec_remove_item)]

struct Store{
  name: &'static str,
  workers: Vec<Worker>,
}

#[derive(Debug, PartialEq)]
struct Worker{
  name: &'static str, 
  age: i32
}

impl Store {
  fn hire_worker(&mut self, worker: Worker) {
    let Worker{name, age} = worker;
    self.workers.push(worker);
    println!("{} ({}) added!", name, age);
  }

  fn fire_worker(&mut self, worker: Worker) {
    self.workers.remove_item(&worker);
    println!("{} removed", worker.name);
  }
}

fn main() {
  let mut store = Store{name:"test", workers:Vec::new()};
  println!("{}", store.name);
  store.hire_worker(Worker{name:"Mary", age:16});
  store.hire_worker(Worker{name:"Bob", age:18});
  println!("{:?}", store.workers);
  store.fire_worker(Worker{name:"Bob", age:18});
  println!("{:?}", store.workers);
}