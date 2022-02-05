mod structs;


use rand::{Rng, thread_rng};
use std::cmp::Ordering;
use std::io::stdin;
use structs::User;
use crate::structs::build_user;

fn main() {
    println!("Please enter name!");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read name!");
    let mut user: User = build_user(name);

    let secret = thread_rng().gen_range(1..101);
    println!("Have chosen a secret number between 1 and 100 , try and guess by entering number");
    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number , you had entered {}", guess);
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => {
                user = structs::update_count(user.guess_count + 1, &user);
                println!("Psst small!")
            }
            Ordering::Greater => {
                user = structs::update_count(user.guess_count + 1, &user);
                println!("Psst large!")
            }
            Ordering::Equal => {
                println!("Bingo {} , you've taken {} guesses!!!", user.name, user.guess_count);
                break;
            }
        }
    }
}
