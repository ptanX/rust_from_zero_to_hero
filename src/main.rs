use rand::Rng;

use crate::dynamic_dispatch::object_trait::{Animal, Cat, Dog};
use crate::struct_learning::basic_structure::User;

mod greeting;
mod dynamic_dispatch;
mod struct_example;
mod struct_learning;

static D: fn(i32) -> i32 = |i: i32| -> i32 {
    i * i
};

fn main() {
    // For struct learning
    let first_user: User = User {
        active: true,
        username: String::from("some username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user_email = first_user.email;
    println!("first user email is: {}", user_email);
    println!("is user active: {}", first_user.active);
    println!("first username is: {}", first_user.username);
    println!("first sign_in_count is: {}", first_user.sign_in_count);
}

