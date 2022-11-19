use rand::Rng;
use std::io;

fn get_guess() -> i16 {
    let mut user_guess = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Enter a valid input");

    return user_guess
        .trim()
        .parse()
        .expect("Input is not a valid number");
}

fn generate_rn() -> i16 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);
    return random_number;
}

fn main() {
    let random_number = generate_rn();
    loop {
        println!("Guess a number between 1 and 10");
        let guessed_number = get_guess();
        if random_number > guessed_number {
            println!("Number is too small")
        } else if random_number < guessed_number {
            println!("Number is too big")
        } else {
            println!("You guessed it! {}", random_number);
            break;
        }
    }
}
