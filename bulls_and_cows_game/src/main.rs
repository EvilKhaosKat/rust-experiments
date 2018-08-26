extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct Number(char, char, char, char);

fn main() {
    println!("Bulls and cows game");
    println!("-------------------");

    let secret = generate_secret_number();
    println!("{:?}", secret)
}

fn generate_secret_number() -> Number {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let used_digits: &mut Vec<char> = &mut Vec::new();

    let first_digit = generate_digit(&digits, used_digits);
    let second_digit = generate_digit(&digits, used_digits);
    let third_digit = generate_digit(&digits, used_digits);
    let fourth_digit = generate_digit(&digits, used_digits);

    Number(first_digit, second_digit, third_digit, fourth_digit)
}

fn generate_digit(numbers: &[char; 10], used_numbers: &mut Vec<char>) -> char {
    loop {
        let num = rand::thread_rng().choose(numbers).unwrap();
        if !used_numbers.contains(num) {
            used_numbers.push(*num);
            return *num
        }
    };
}
