use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_number = rand::thread_rng().gen_range(1..=100);

    // print!("Rand Number: {rand_number}");

    println!("\n## Guess the Number! ##");
    println!("\nEnter your guess: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("\nYou know this is not a number!?\n");
                println!("\nEnter another guess (Number please): ");
                continue;
            }
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("\nToo Small!"),
            Ordering::Greater => println!("\nToo Big!"),
            Ordering::Equal => {
                println!("\nYou Win!");
                break;
            }
        }

        println!("\nYour Guess: {guess}");
        println!("\nEnter another guess: ");
    }
}
