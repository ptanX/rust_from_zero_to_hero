mod greeting;
mod dynamic_dispatch;
mod guess;
mod struct_example;

use std::cmp::Ordering;
use crate::dynamic_dispatch::object_trait::{Animal, Cat, Dog};
use std::io;
use rand::Rng;

static D: fn(i32) -> i32 = |i: i32| -> i32 {
    i*i
};
fn create_animal(kind: &str) -> Box<dyn Animal> {
    if kind == "DOG" {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
