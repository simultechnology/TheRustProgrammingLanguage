use std::{cmp::Ordering, io};
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secrete number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        if guess.eq("quit") {
            break;
        }
        let guess= match guess.parse() {
            Ok(num) => num,
            Err(_) => panic!("guess value must be number.")
        };

        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}