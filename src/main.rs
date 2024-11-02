use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess the number!\nPlease input your guess.");
    
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let your_guess: u32 = guess.trim().parse().expect("Please type a number!");
        if your_guess > secret_number {
            println!("Your number is bigger than the answer.")
        } else if your_guess < secret_number {
            println!("Your number is smaller than the answer.")
        }else {
            println!("Correct!");
            break;
        }
    }
}
