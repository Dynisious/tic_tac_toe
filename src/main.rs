//! since this is a `binary` crate, meaning it is a program not a library, `main.rs` is
//! the entry point of the program. Much wow. Very surprise.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2017/12/4

//This is an example of an inline comment same as many other languages.

//Documentation in Rust supports the Markdown format. I'll let you look that up if you're interested.  
//A funny thing about Markdown is that if you put things on seperate lines with no empty
//lines in-between, like the above documentation, Markdown will concatenate them as one
//line. To seperate the lines end the line with a double space ("  ").

//"//!" Specifies that this comment block is for the namespace it is inside (the root namespace in this instance.)
//"///" Specifies that this comment block is for the next item it comes across (the main function in this instance.)
//The difference between using this and "//!" or "///" is that `cargo doc` will convert
//the other two comments into documentation where as "//" is ignored.

//This declares a new Rust module which is a new namespace in this program.
//When this program builds it will expect to find one of two things:
//+ `square.rs` in the same folder as `main.rs` which defines the `square` module.
//+ `square/mod.rs` a folder named `square` in the same folder as `main.rs` where `mod.rs` defines the `square` module.
mod temp;

//This imports all of the public elements of temp into this namespace.
use temp::*;

/// The entry point of a Rust program.
fn main() {
    //This function takes no parameters and returns nothing (technically it returns an empty tuple (), but that is nothing)
    println!("Hello, world!");
}
