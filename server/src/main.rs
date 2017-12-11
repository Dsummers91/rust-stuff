#![feature(plugin)]
#![plugin(rocket_codegen)]

mod person;

use person::Person;

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{Json, Value};

use rocket::response::NamedFile;

use std::io;
use std::path::{Path, PathBuf};

#[get("/<name>")]
fn name(name: String) -> Json<Value> {
    Person::new(&name).json()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/hello.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index, name]).launch();
}