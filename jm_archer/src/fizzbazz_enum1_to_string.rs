/*
cargo run --bin fizzbazz_enum1_to_string
*/
// use let .. if

enum FBItems {
    FizzBazz,
    Fizz,
    Bazz,
    Value(i32),
}

impl ToString for FBItems {
     fn to_string(&self) -> String {
        match *self {
            FizzBazz => "\tFizzBazz".to_string(),
            Fizz => "\t\tFizz".to_string(),
            Bazz => "\t\t\tBazz".to_string(),
            Value(n) => n.to_string(),
        }
    }
}
use self::FBItems::*;

fn main() {
    for current_number in 1..25 {

        let item = match current_number {
            current_number if current_number % 15 == 0 => { FizzBazz }
            current_number if current_number % 5 == 0 => { Fizz }
            current_number if current_number % 3 == 0 => { Bazz }
            current_number => Value(current_number),
        };
        println!("{}", item.to_string());

    }
}