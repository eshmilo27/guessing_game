use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();
    //println!("The secret number is: {secret_number}");

    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Number of guesses: {guess_count}");
                break;
            }
        }
    }
}
