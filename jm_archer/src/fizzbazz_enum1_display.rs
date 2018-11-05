/*
cargo run --bin fizzbazz_enum1_display
*/
// use let .. if

enum FBItems {
    FizzBazz,
    Fizz,
    Bazz,
    Value(i32),
}

use self::FBItems::*;
use std::fmt::{Display, Formatter};
use std::fmt::Error;

impl Display for FBItems {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            FizzBazz => f.write_str("\tFizzBuzz"),
            Fizz => f.write_str("\t\tFizz"),
            Bazz => f.write_str("\t\t\tBuzz"),
            Value(num) => write!(f, "{}", num),
        }
    }
}

fn main() {
    for current_number in 1..25 {

        let item = match current_number {
            current_number if current_number % 15 == 0 => { FizzBazz }
            current_number if current_number % 5 == 0 => { Fizz }
            current_number if current_number % 3 == 0 => { Bazz }
            current_number => Value(current_number),
        };
        println!("{}", item);

    }
}