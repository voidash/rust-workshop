**Create a New Project:**

* In Rust, projects are organized into packages and crates. You can start by creating a new Rust project using the `cargo` tool, which comes with Rust.

* Open your terminal and navigate to the directory where you want to create your project. If you are on windows then use `powershell`
  
  `cargo new hello_world`
  
  This command will create a new directory called `hello_world` with the necessary project structure.

* **Edit the Main File:**
  
  * Navigate into the `hello_world` directory:

`cd hello_world`

* Inside this directory, you'll find a file named `main.rs`. Open this file in a text editor of your choice.

* Replace the existing code with the following Hello program:

````
fn main() {     
	println!("Hello, 🇳🇵!");
}
````

````
This code defines a `main` function, which is the entry point of your Rust program. It uses the `println!` macro to print "Hello, 🇳🇵!" to the console.
````

* **Build and Run:**
  
  * Now, you can build and run your Rust program using `cargo`. In your terminal, run:
  `cargo run`
  
  This command will compile and execute your Rust program. You should see the output "Hello, 🇳🇵!" displayed in the terminal.

This simple program confirms that your Rust development environment is set up correctly and ready for more advanced development. 
