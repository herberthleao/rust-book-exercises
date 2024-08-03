mod header;

use std::io;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use rand::Rng;

use header::draw_header;

fn main() {
    run(generate_secret_number());
}

/// Run the game.
///
/// # Arguments
///
/// * `secret_number` - The number to be guessed.
fn run(secret_number: u32) {
    loop {
        draw_header("Guess The Number");

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => show_result("Too small!"),
            Ordering::Greater => show_result("Too big!"),
            Ordering::Equal => {
                show_result(&format!("You win! The number is: {secret_number}."));
                break;
            }
        }
    }
}

/// Generates the secret number.
/// 
/// # Returns
/// 
/// The secret number to be guessed.
fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

/// Shows the result message.
/// 
/// # Arguments
/// 
/// * `message` - The result message.
fn show_result(message: &str) {
    println!();
    println!("{}", message);
    thread::sleep(Duration::from_millis(1500));
}