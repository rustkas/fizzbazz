/*
cargo itemun --bin fizzbazz_enum1
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
        let item: FBItems = if current_number % 15 == 0 {
            FizzBazz
        } else if current_number % 3 == 0 {
            Fizz
        } else if current_number % 5 == 0 {
            Bazz
        } else {
            Value(n)
        };

        match item {
            FizzBazz => println!("\tFizzBazz"),
            Fizz => println!("\t\tFizz"),
            Bazz => println!("\t\t\tBazz"),
            Value(n) => println!("{}", current_number),
        }
    }
}