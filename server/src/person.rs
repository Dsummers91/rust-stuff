use rocket_contrib::{Json, Value};

pub struct Person<'a> {
  pub name: &'a str,
  age: u8
}

impl<'a>  Person<'a>  {
  pub fn new(name: &'a str) -> Person<'a>  {
    Person{name: name, age: 18}
  }

  pub fn json(self: &Person<'a>) -> Json<Value> {
    Json(json!({
      "name": self.name,
      "age": self.age
    }))
  }
}