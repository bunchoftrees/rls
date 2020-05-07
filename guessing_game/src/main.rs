use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut counter = 0;

    loop {
        print!("Please input your guess:  ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                counter += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                counter += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                counter += 1;
                println!("You took {} counter to win!", counter);
                break;
            }
        }
    }
}
