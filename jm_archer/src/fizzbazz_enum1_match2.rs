/*
cargo itemun --bin fizzbazz_enum1_match2
*/
// use let .. if

enum FBItems {
    FizzBazz,
    Fizz,
    Bazz,
    Value(i32),
}

use self::FBItems::*;

fn main() {
    for current_number in 1..25 {

        let item = match current_number {
            current_number if current_number % 15 == 0 => { FizzBazz }
            current_number if current_number % 5 == 0 => { Fizz }
            current_number if current_number % 3 == 0 => { Bazz }
            current_number => Value(n),
        };
        match item {
            FizzBazz => println!("\tFizzBazz"),
            Fizz => println!("\t\tFizz"),
            Bazz => println!("\t\t\tBazz"),
            Value(n) => println!("{}", current_number),
        }
    }
}