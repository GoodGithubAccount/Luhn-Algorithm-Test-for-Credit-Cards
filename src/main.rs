use rand::Rng;
use luhn3;
use std::collections::HashSet;
use rayon::prelude::*;
use std::convert::TryInto;
use std::time::{Instant};

const NUM_CARDS: usize = 10000000;

fn main() {
    let start_time = Instant::now();

    (0..NUM_CARDS).into_par_iter()
        .for_each(|_| {
            let valid_credit_card = generate_valid_credit_card_number();
            check_and_print_valid_credit_card_numbers(&valid_credit_card);
        });

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Time taken: {:?}", elapsed_time);
}

fn generate_valid_credit_card_number() -> String {
    let mut rng = rand::thread_rng();
    let iin_length = rng.gen_range(1..7);
    let mut number = String::with_capacity(16);

    for _ in 0..iin_length {
        number.push(std::char::from_digit(rng.gen_range(1..10), 10).unwrap());
    }

    let total_length = 16 - iin_length;
    for _ in 0..total_length {
        number.push(std::char::from_digit(rng.gen_range(0..10), 10).unwrap());
    }

    let checksum = calculate_checksum(&number);
    number.push(std::char::from_digit(checksum as u32, 10).unwrap());
    number
}

fn calculate_checksum(number: &str) -> u32 {
    let ints: Vec<i32> = number.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut sum = 0;
    for (i, &digit) in ints.iter().rev().enumerate() {
        let mut j = digit;
        if i % 2 == 0 {
            j *= 2;
            if j > 9 {
                j -= 9;
            }
        }
        sum += j;
    }
    if sum % 10 == 0 {
        0
    } else {
        (10 - sum % 10).try_into().unwrap()
    }
}

fn check_and_print_valid_credit_card_numbers(valid_credit_card: &str) {
    let mut valid_credit_cards = HashSet::new();
    valid_credit_cards.insert(valid_credit_card.to_string());
    let chars: Vec<char> = valid_credit_card.chars().collect();
    for i in 0..16 {
        for j in 0..10 {
            if chars[i] == std::char::from_digit(j, 10).unwrap() {
                continue;
            }
            let mut modified_credit_card = chars.clone();
            modified_credit_card[i] = std::char::from_digit(j, 10).unwrap();
            let modified_str: String = modified_credit_card.iter().collect();
            if luhn3::valid(modified_str.as_bytes()) {
                valid_credit_cards.insert(modified_str.clone());
                println!("ORIGINAL:{} OTHER: {}", valid_credit_card, modified_str);
            }
        }
    }
}
