/*
    A tuple is a collection of values of different types.  Tuples are constructed using parenthesis `()`, and
    each tuple itself is a value with type signature `(T1, T2, ...)`, where `T1`, `T2` are the types of its
    members.  Functions can use tuples to return multiple values, as tuples can hold any number of
    values.
*/

use std::fmt;

// Tuples can be used as function arguments and as return values
fn _reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of tuples to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// The following struct is for the Activity
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )",
        self.0, self.1, self.2, self.3)
    }
}


fn main() {
    let long_tuple = (  1u8, 2u16, 3u32, 4u64,
                        -1i8, -2i16, -3i32, -4i64,
                        'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // Tested, Debug does not implement over a certain amount of

    let pair = (1, true);
    println!("pair is {:?}", pair);

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parenthesis
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}
