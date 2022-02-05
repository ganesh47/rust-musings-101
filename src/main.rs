use std::io;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret = thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number , you had entered {}", guess);
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Its small"),
            Ordering::Greater => println!("Its large"),
            Ordering::Equal => {
                println!("Bingo!!");
                break;
            }
        }
    }
}
