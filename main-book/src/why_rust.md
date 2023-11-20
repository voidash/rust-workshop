## What is Rust? {#what-is-rust}

- For Systems Programming
- Targets the same use cases as C/C++ and more
- Compiled Language (uses LLVM-IR)
- Static Typed Language
- Supports many platforms
  - x86, ARM, WebAssembly, RISC-V
- Can be used in Desktops, mobiles, routers, servers, etc.

[Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)

## Selling Points of Rust

- Good Tooling
- Great compiler errors
- Built-in support for testing
- Nice LSP (Language Server Protocol)
- Cargo: The Rust package Manager and Build system

## Memory Safety

- Ensures safety through compiler enforcement
- No null pointers, no dangling pointers
- No data races in concurrent situations inside safe code
- Concept of ownership and borrowing ensures memory safety at compile time

## Memory Management

- Resources managed through RAII (Resource Acquisition Is Initialization)
- Explicit heap allocation with 'Box'

[More on Memory Management](https://deepu.tech/memory-management-in-rust/)


## Rust Code Examples

### Hello World

```rust
fn main() {
    println!("Hello world");
}
```

### FizzBuzz

```rust
fn main() {
    for num in 1..=100 {
        match (num % 3, num % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", num),
        }
    }
}
```

### Rust vs. C++ 

The following C++ code compiles but it doesn't work as intended. can you guess the output?

- C++ Code
  ```cpp
  #include <iostream> 
  #include <vector>
  #include <string>

  int main() {
      std::vector<std::string> data;
      data.push_back("hello");
      std::string &adt = data[0];
      data.push_back("world");
      std::cout << adt;
      return 0;
  }
  ```
  If you went through the code then one thing you will see is that the output is blank. The main reason is that `std::vector` which is a dynamically sized array. We haven't explicitly provided the size nor the contents for the vector so it starts with size of `1`. As you start adding more than one items in array, the array has to be resized. Vector in C++ resizes by creating a new array with size greater than the current one, and then copies the contents of existing array to new one. `&adt` stores the reference to the first element in the array, However line below it causes the array to resized. The resizing operation causes the internal array address to be changed, as the existing array is copied to new array with bigger size, and the existing array contents are deleted. This means the address that `adt` holds, if you dereference it, it will contain nothing. So the output will be **blank**


- Rust Equivalent
  ```rust
  fn main() {
      let mut data: Vec<&str> = Vec::new();
      data.push("Hello");
      let adt = &data[0];
      data.push("World");
      println!("{adt}");
  }
  ```
  This code doesn't compile on Rust, because if you have a reference to something and if you try to modify the content before discarding the reference then the reference might not be memory safe so Rust doesn't allow you. 


## Advanced Topics in Rust

- Move Semantics
- Borrowing and Borrow Checker
- Algebraic Data Types / Enums
- Zero Cost Abstraction
- Pattern Matching
- Generics
- Safe and correct memory management, without a garbage collector

## Rust Build System and Package Manager

- Cargo Commands
- Scalar Types
- Compound Types

## Rust in Action

- Writing an OS in Rust: [Phil-Opp's Blog](https://os.phil-opp.com/)
- Kernel Driver with Rust: [Kernel Driver Tutorial](https://not-matthias.github.io/posts/kernel-driver-with-rust/)

## Real World Rust Projects

- Tor's implementation in Rust ("arti")
- Dropbox's file sync engine
- Figma's Multiplayer Server
- Coursera's grading Server

## Rust as a System Language

- Low level control
  - Unsafe blocks
  - Foreign function interface
  - Inline assembly
- Zero Cost abstractions
- Pointer Types
