extern crate rand;

use std::io;
use rand::Rng;

#[derive(Debug)]
struct Number(char, char, char, char);

impl Number {
    fn from_correct_string(input_string: String) -> Number {
        let chars: Vec<char> = input_string.chars().collect();
        Number(chars[0], chars[1], chars[2], chars[3])
    }
}

fn main() {
    println!("Bulls and cows game");
    println!("-------------------");

    let secret = generate_secret_number();

    loop {
        println!("Please input your 4 number guess");

        let guess = read_input();
        if guess.len() != 4 {
            continue;
        }

        let input_number = Number::from_correct_string(guess);
        println!("input: {:?}", input_number)
    }
}

fn read_input() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read used input line");

    String::from(guess.trim())
}

fn generate_secret_number() -> Number {
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut used_digits: Vec<char> = Vec::new();

    let first_digit = generate_digit(&digits, &mut used_digits);
    let second_digit = generate_digit(&digits, &mut used_digits);
    let third_digit = generate_digit(&digits, &mut used_digits);
    let fourth_digit = generate_digit(&digits, &mut used_digits);

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
