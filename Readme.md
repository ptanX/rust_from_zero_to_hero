# Rust Learning From Zero To Hero

## 1 Struct

### Defining And Instantiating Struct
1. Define **Struct** keyword and name of structure: 
2.  inside curly brackets, we define the names and types of the pieces of data, which we call fields.
Ex: 
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
# 2 Enums and Pattern Matching
## Enum Definition
Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.<br>
We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are the variants of the enum:
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
We can put value directly into each enum variant like below: <br>
- Putting `String` inside specific `Enum`:
```rust
enum IpAddrWithStringValue {
        V4(String),
        V6(String),
    }
```

- Putting different types and amounts of associated data to each variant: <br>
```rust
enum IpAddrWithArbitraryValue {
        V4(u8, u8, u8, u8),
        V6(String),
    }
```
- Embedding the address data inside the variants in the form of two different structs: <br>
```rust
struct Ipv4AddrValue {
    // --snip--
}

struct Ipv6AddrValue {
    // --snip--
}

enum IpAddrWithStructValue {
    V4(Ipv4AddrValue),
    V6(Ipv6AddrValue),
}
```


## Option Type
- The Option type represents an optional value: it can either be `Some` (containing a value) or `None` (indicating absence).
- Rust uses Option to handle potentially missing or invalid values, avoiding null pointer errors.
- You’ll encounter functions in Rust that return Option types.Use Some(value) to wrap a value, or None to represent absence: <br>
```rust
let maybe_number: Option<i32> = Some(42);
```
- Use pattern matching to handle Option values explicitly.
```rust
match maybe_number {
    Some(value) => println!("Value: {}", value),
    None => println!("No value"),
}
```
- Rust provides useful methods for working with Option:
    -  `unwrap()`: Extracts the value (panics if None).
    -  `unwrap_or(default)`: Returns the value or a default.
    -  `map(func)`: Transforms the value.
    -  `and_then(func)`: Applies a function returning another Option.
    -  `filter(predicate)`: Filters based on a condition.
## Matching pattern
## if let construct

# 3 Manage Growing Project With Packages, Crates, Modules
# 4 Rust Collections
# 5 Error Handling
# 6 Generic Types, Traits, Lifetimes
# 7 Smart Pointer
# 8 Concurrency
# 9 Building Multi Threads Web Application