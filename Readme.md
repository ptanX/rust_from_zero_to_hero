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

# 3 Understanding Ownership
## What Is Ownership?
## References and Borrowing
### Mutable Borrowing
### Lifetime and Scope

# 4 Manage Growing Project With Packages, Crates, Modules
Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates 
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and use: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module
## Packages And Crate
- A **crate** is the smallest amount of code that the Rust compiler considers at a time.
- **Binary crates** are programs you can compile to an executable that you can run,
such as a command-line program or a server. Each **must have a function called main** that defines what happens when the executable runs.
- **Library crates don’t have a main function**, and they don’t compile to an executable. Instead,
they define functionality intended to be shared with multiple projects.
- A package is **a bundle of one or more crates that provides a set of functionality**.
A package contains a Cargo.toml file that describes how to build those crates.
## Defining Modules to Control Scope and Privacy
### Modules Cheat Sheet

Before we get to the details of modules and paths, here we provide a quick
reference on how modules, paths, the `use` keyword, and the `pub` keyword work
in the compiler, and how most developers organize their code. We’ll be going
through examples of each of these rules throughout this chapter, but this is a
great place to refer to as a reminder of how modules work.

- **Start from the crate root**: When compiling a crate, the compiler first
  looks in the crate root file (usually *src/lib.rs* for a library crate or
  *src/main.rs* for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules;
  say you declare a “garden” module with `mod garden;`. The compiler will look
  for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod
    garden`
  - In the file *src/garden.rs*
  - In the file *src/garden/mod.rs*
- **Declaring submodules**: In any file other than the crate root, you can
  declare submodules. For example, you might declare `mod vegetables;` in
  *src/garden.rs*. The compiler will look for the submodule’s code within the
  directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
  - In the file *src/garden/vegetables.rs*
  - In the file *src/garden/vegetables/mod.rs*
- **Paths to code in modules**: Once a module is part of your crate, you can
  refer to code in that module from anywhere else in that same crate, as long
  as the privacy rules allow, using the path to the code. For example, an
  `Asparagus` type in the garden vegetables module would be found at
  `crate::garden::vegetables::Asparagus`.
- **Private vs. public**: Code within a module is private from its parent
  modules by default. To make a module public, declare it with `pub mod`
  instead of `mod`. To make items within a public module public as well, use
  `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
  items to reduce repetition of long paths. In any scope that can refer to
  `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
  crate::garden::vegetables::Asparagus;` and from then on you only need to
  write `Asparagus` to make use of that type in the scope.

## Bringing Paths into Scope with the use Keyword
## Separating Modules into Different Files



# 5 Rust Collections
# 6 Error Handling
# 7 Generic Types, Traits, Lifetimes
# 8 Smart Pointer
## Running Code on Cleanup with the Drop Trait
You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.

We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer.<br>
For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

In some languages, for some types, the programmer must call code to free memory or resources every time they finish using an instance of those types.<br>
Examples include file handles, sockets, or locks. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically.
```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
Result:
```
cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.60s
     Running `target/debug/drop-example`
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```
# 9 Concurrency
Here are the topics we’ll cover in this chapter:

- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library
## Using Threads to Run Code Simultaneously
- To create a new thread, we call the thread::spawn
- saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
# 10 Building Multi Threads Web Application
Here is our plan for building the web server:

- Learn a bit about TCP and HTTP.
- Listen for TCP connections on a socket.
- Parse a small number of HTTP requests.
- Create a proper HTTP response.
- Improve the throughput of our server with a thread pool.