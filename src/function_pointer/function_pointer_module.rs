fn add_two_number_function(first_number: i32, second_number: i32) -> i32 {
    return first_number + second_number
}
pub static  ADD_TWO_NUMBER: fn(i32, i32) -> i32 = add_two_number_function;