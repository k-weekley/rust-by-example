/*
    Implementing `fmt::Display` for a structure where the elements must each be handled sequentially
    is tricky.  The problem is that each `write!` generates a `fmt::Result`. Proper handling of this
    requires dealing with all of the results.  Rust provides the `?` operator for this exact purpose.
*/

// Using `?` on `write!` looks like this:

// Try `write!` to see if it errors.  If it errors, return
// the error.  Otherwise, continue.
// write(f, "{}", value)?;

/*
    Alternatively, you can also use the `try!` macro, which works the same way.  This is a bit more
    verbose and no longer recommended, but you may still see it in older Rust code.  Using `try!` looks
    like this:
*/

// try!(write!(f, "{}", value));

// With `?` available, implementing `fmt::Display` for a `Vec` is straightforward:

use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the interation
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
