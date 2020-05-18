// Any program requires comments, and Rust supports a few different varieties:

// // Line comments which go to the end of the line.
// /* Block comments which go to the closing delimiter. */

// /// Generate library docs for the following item.
// //! Generate library docs for the enclosing item.


fn main() {
    // This is an example of a line comment
    // There are two slashes at the beginning of the Line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    /*
     * This is another type of comment, a block comment.  In general,
     * line comments are the recommended comment style.  But
     * block comments are extremely useful for temporarily disabing
     * chunks of code.  /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

     /*
     Note: the previous column of `*` was entirely for style.  There's
     no actual need for it.
     */

     // You can manipulate expressions more easily with block comments
     // than with line comments. Try deleting the comment delimiters
     // to change the result:
     let x = 5 + /* 90 + */ 5;
     println!("is `x` 10 or 100? x = {}", x);
}
