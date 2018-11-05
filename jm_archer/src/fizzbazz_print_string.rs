/*
cargo itemun --bin fizzbazz_print_string
*/
// use let .. if

fn main() {
    for current_number in 1..25 {

        let s: String = if current_number % 15 == 0 {
                            "\tFizzBazz".to_string()
                        } else if current_number % 3 == 0 {
                            "\t\tFizz".to_string()
                        } else if current_number % 5 == 0 {
                            "\t\t\tBazz".to_string()
                        } else {
                            current_number.to_string()
                        };

        println!("{}", s);
    }
}