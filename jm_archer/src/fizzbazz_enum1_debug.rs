/*
cargo run --bin fizzbazz_enum1_debug
*/
// use let .. if

enum FBItems {
    FizzBazz,
    Fizz,
    Bazz,
    Value(i32),
}

use self::FBItems::*;
use std::fmt::{Debug, Formatter, Result};

impl Debug for FBItems {

    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FizzBazz => write!(f, "\tFizzBuzz"),
            Fizz => write!(f, "\t\tFizz"),
            Bazz => write!(f, "\t\t\tBuzz"),
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
        println!("{:?}", item);
    }
}