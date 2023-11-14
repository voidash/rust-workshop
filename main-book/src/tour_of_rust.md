Some snippets that will help you understand Rust code and some Rust fundamentals along the way

`let` introduces variable binding.

````rust
let a = 42; 
````

Since Rust is statically typed language, which means you have to specify the type of variable. i.e if it is storing strings, or integers or floating numbers or your own data type

````rust

let a: i32 = 12; // 32 bit integer 
let b: &str = "Hello nepal";  //str
let c: char = 'a'; //char
let d: bool = true;
let e: f32 = 3.1415;
let c: u32 = 32; //32 bit integer but you can't store negative numbers
````

Rust can implicitly identify your variable type based upon the content you are passing to variable.

````rust
let a = 12; // automatically assigned to i32
````

To print something to console, you just use 

````rust
println!("Hello world");
````

to print the contents of the variable you can

````rust
let age : i32 = 21;
println!("Hello, my age is {}", age);
// or you can also 
println!("Hello, my age is {age}");
````

Notice that exclamation mark at the end. it means it is [Rust macros](Rust%20macros.md) . 

Rust variables are immutable by default which means, you can't change the content of the variable once you assign something to it. 

````rust
let age : i32 = 21;
age = 22; // error this won't work, because age is immutable
````

You can shadow a variable bindings

````rust
let x = 17;
let x = "awesome"; // x is not mutable, but we re-binded it
````

You can also use patterns to declare variables

````
let (a,b) = ("Nepal GPO sucks",12);
````

You have to explicitly use the keyword `mut` to make it re-assignable

````rust
let mut age: i32 = 21;
age = 22; // valid because we used mut keyword
````

Now similarly generate small snippets and information combination for
conditionals, loops, match, struct, vector, array, tuples and so on

#### Comments

````
/// Triple-slash comments are docstring comments.
///
/// `rustdoc` uses docstring comments to generate
/// documentation, and supports **Markdown** formatting.
fn foo() {
    // Double-slash comments are normal.

    /* Block comments
     * also exist /* and can be nested! */
     */
}
````

#### Conditionals

In Rust, you can use `if`, `else if`, and `else` statements for conditionals.

````rust
let number = 7;

if number < 5 {
    println!("Condition is true");
} else if number == 7 {
    println!("Number is seven");
} else {
    println!("Condition is false");
}
````

#### Arrays

* Arrays are generically of type `[T; N]`.
  * T is type, can be &str, char, bool, i32 etc
  * N is a compile-time *constant*. Arrays cannot be resized.
  * Array access is bounds-checked at runtime.
* Arrays are indexed with `[]` 
  * `arr[3]` gives you the 4th element of `arr`

````
let arr1 = [1, 2, 3]; // (array of 3 elements)
let arr2 = [2; 32];   // (array of 32 `2`s)
let arr2: [u8;4] = [12,32,73,11]; // (array of 4 8-bit integers that can only accept positive numbers)
````

#### Slices

You create a slice of array and it is in format &\[T\]. They are reference to your arrays, and you can take specific section of your arrays with slices too. 

````
let arr = [0, 1, 2, 3, 4, 5];
let total_slice = &arr;         // Slice all of `arr`
let total_slice = &arr[..];     // Same, but more explicit
let partial_slice = &arr[2..5]; // [2, 3, 4]
````

#### Strings

There are two types of strings.`String` and `&str`. Strings are growable set of characters. Basically you can think of it as dynamic array of characters. `&str` is the slice of `String` which means they are fixed. For more information look to [stack vs heap](stack%20vs%20heap.md), to understand the difference between String and &str. 

When you use literals in your program, compiler automatically assumes it is of type `&str`

````rust
let a = "Nepal's General post office is why i hate government work";
// this is of type &str 

let b = String::from("Foo"); //this is of type String
let s4: &str = &b; 
````

#### Vector

`Vec<T>` are resizable arrays. `<T>` denotes the generic type, so we pass the type from there. 

````rust
let mut v: Vec<i32> = Vec::new(); // Declare a new vector

v.push(5);
v.push(6);
v.push(7);
````

Shortcut method to create vector

````
let v2 = vec![1,2,3];
````

Vectors can be indexed with `[]`, but you have to use `usize`.

````
let v2 = vec![1,2,3];
let i: u8 = 2;
// let d = v2[i]; // error as i should be of type usize
let d = v2[i as usize]; // casting
````

#### Loops

Rust provides several types of loops, including `loop`, `while`, and `for`.

Infinite Loop

````rust
loop {
    println!("This will loop forever until explicitly broken!");
    break; // Use break to exit the loop
}
````

#### While Loop

````rust
let mut count = 0;

while count < 5 {
    println!("count: {}", count);
    count += 1;
}
````

#### For Loop

````rust
for number in 1..4 { // 1..4 is a range (1,2,3)
    println!("number: {}", number);
	// you can break and continue too
}

````

notice that `1..4`. It is a range. The syntax is `start..end` , Some operation on ranges

````rust
	// including all the endpoints of range
	let include_all = (1..=4); // 1,2,3,4 

	// reversing
	let reversed = (1..4).rev(); // 3,2,1

	// convert range to vector
	let numbers = (1..4).collect::<i32>(); // collect 1,2,3,4 as 32 bit integer 
	// or
	let numbers: Vec<i32> = (1..4).collect();

	//skip
	let skipped_val = (1..10).skip(2);  //1,3,5,7,9

	// explore enumerate(), iter(), take() by yourself
````

## Everything is expressions (almost)

Expressions return a value. So Expressions are something that returns value, can't get simpler than that.  
We can bind many things to variable name, because everything is expression. The rule is, if it's expression then we can bind it to a variable.

````
let x = -5;
let y = if x > 0 { "greater" } else { "less" };
println!("x = {} is {} than zero", x, y);
````

#### Struct

Structs are used to create custom data types.

````rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("coldplay@gmail.com"),
    username: String::from("yellow"),
    active: true,
    sign_in_count: 1,
};
````

#### Tuples

Tuples are fixed-size collections of multiple types.

````rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup; // Destructuring

println!("The value of y is: {}", y);
````

#### Functions

Functions in Rust are defined using the `fn` keyword, with typed parameters and an optional return type.

````rust
fn square(n: i32) -> i32 {
    n * n // no need to type return 
    // if there is no semicolon at the end then it means it returns that value
}

fn squareish(n: i32) -> i32 {
    if n < 5 { return n; } // if you want to return early then use return keyword
    n * n
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

greet("Alice");
````

You can store functions in variable

````
let x : fn(i32) -> i32 = square;

fn do_it_two_times(f: &fn(i32) -> i32, x: i32 ) -> i32 {
	f(f(x))
}

let y = do_it_two_times(&square, 5);
````

#### Match

`match` is similar to switch cases in other languages, but more powerful.

````rust
let number = 2;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Anything else"), // '_' is the default case
}
````

you can match with range

````rust
let number = 13;

match number {
    1..=5 => println!("It's between 1 and 5"),
    6..=10 => println!("It's between 6 and 10"),
    _ => println!("It's something else"),
}
````

you can add guards to match

````rust
let number = 4;

match number {
    n if n % 2 == 0 => println!("Even number"),
    n if n % 2 != 0 => println!("Odd number"),
    _ => println!("It's something else"),
}

````

you can match with Enums too, see below examples

#### Enums

Enums are a way to define a type by enumerating its possible values.

````rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_direction(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up"),
        // Other directions...
    }
}
````

Enums in Rust can also have data associated with their variants. This feature can be particularly useful for representing more complex scenarios like error handling or state management.

````rust
enum AppState {
    Loading,
    Content(String),
    Error(String),
}

fn get_app_state(loading: bool, has_error: bool) -> AppState {
    if loading {
        AppState::Loading
    } else if has_error {
        AppState::Error(String::from("An error occurred"))
    } else {
        AppState::Content(String::from("Content loaded successfully"))
    }
}

let state = get_app_state(false, true);

match state {
    AppState::Loading => println!("App is loading..."),
    AppState::Content(content) => println!("Content: {}", content),
    AppState::Error(error) => println!("Error: {}", error),
}
````

This example shows an `AppState` enum with different variants for loading, content, and error states. The `match` statement then handles each state accordingly.

Rust's `Option` and `Result` types are actually enums under the hood. Understanding the basic enum concept can help grasp these types better.

* `Option<T>` can be either `Some(T)` or `None`.
* `Result<T, E>` can be either `Ok(T)` for success or `Err(E)` for an error.

Here's a simplified version of how they might be defined:

````rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
````

````rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

let result = divide(10.0, 2.0);
````

As written above, `Result<T, E>` is used for error handling. It returns `Ok(value)` if successful, or `Err(error)` if an error occurs.

````rust
fn check_age(age: i32) -> Result<(), String> {
    if age >= 18 {
        Ok(())
    } else {
        Err(String::from("Underage"))
    }
}

let age_check = check_age(20);
````

#### Cargo

* Create a new project:
  * `cargo new project_name` (library)
  * `cargo new project_name --bin` (executable)
* Build your project: `cargo build`
* Run your tests: `cargo test`

#### Cargo.toml

* This is what Rust uses to manage dependencies and project metadata. When you open something like this is shown

````
[package]
name = "my_project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
serde = "1.0"
serde_json = "1.0"
log = "0.4"

````

* You add crates to your project as dependency

````
[dependencies] 
futures = "0.3" // name = version
````

* **Exact version**: `"1.2.3"`
* **Caret version requirements**: `"^1.2.3"` (default, compatible with public API version 1.2.3)
* **Tilde version requirements**: `"~1.2.3"` (version 1.2.3 and the versions up to 1.3.0, not including 1.3.0)
* **Wildcard version requirements**: `"1.2.*"` (any version that starts with 1.2)

