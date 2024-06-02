# Rust Learning From Zero To Hero

## Struct

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
