/*
cargo itemun --bin fizzbazz_just_print
*/

fn main() {
    for current_number in 1..25 {
        if current_number % 15 == 0 {
            println!("\tFizzBazz");
        } else if current_number % 3 == 0 {
            println!("\t\tFizz");
        } else if current_number % 5 == 0 {
            println!("\t\t\tBazz");
        } else {
            println!("{}", current_number);
        };
    }
}