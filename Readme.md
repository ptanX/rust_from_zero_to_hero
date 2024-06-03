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
## Matching pattern
## if let construct

# 3 Manage Growing Project With Packages, Crates, Modules
# 4 Rust Collections
# 5 Error Handling
# 6 Generic Types, Traits, Lifetimes
# 7 Smart Pointer
# 8 Concurrency
# 9 Building Multi Threads Web Application