use rand::Rng;

use crate::dynamic_dispatch::object_trait::Animal;
use crate::enum_learning::ip_addr_enum::{create_ip_addr_with_string, get_ip_addr_with_string_value};
use crate::struct_learning::basic_structure::User;

mod greeting;
mod dynamic_dispatch;
mod struct_learning;
mod enum_learning;

static D: fn(i32) -> i32 = |i: i32| -> i32 {
    i * i
};

fn main() {
    // For struct learning
    println!("################### Struct Learning ###################");
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
    // for enum learning
    println!("################### Enum Learning ###################");
    let ip_kind = String::from("V4");
    let ip_v4_value = String::from("127.0.0.1");
    let ip_addr_value = create_ip_addr_with_string(&ip_kind, &ip_v4_value);
    let ip_value = get_ip_addr_with_string_value(&ip_addr_value);
    println!("ip value is: {}", ip_value)
}



