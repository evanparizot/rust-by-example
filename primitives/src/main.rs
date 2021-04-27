mod arrays_slices;

use std::fmt;
use std::fmt::Formatter;

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_int = 7;

    // variables immutable by default. have to modify with 'mut'
    let mut inferred_type = 12;
    inferred_type = 123456789;

    let mut mutable = 12;
    mutable = 13;

    // mutable = true; // Illegal reassignment

    let mutable = true; //shadowing
}

fn literals_and_operators() {
    // 1_000 == 1000
    // 0.000_001 == 0.000001

    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);
}

#[derive(Debug)] // auto creates implementation to make struct printable with 'fmt::Debug'
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1);
        write!(f, "({} {})", self.2, self.3)
    }
}

fn tuples() {

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    let pair = (1, true);
    println!("reversed: {:?}", reverse(pair));

    let one_item_tuple = (5u32,);
    let normal_int = (5u32);

    // Destructuring tuples
    let tuple = (1, "yellow", 5.2, false);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}