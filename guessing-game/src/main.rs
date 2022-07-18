/**
 *
 * Guessing CLI game with Rustlang
 *
 * Objectives: generate a random number from 0 - 10
 *
 * Have two players guess the number. The one closest to the number wins.
 * A scoreboard will keep track of the winners
 *
 */
use rand::Rng;
use std::io;

fn get_guess() -> i16 {
    let mut user_guess = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    return user_guess
        .trim()
        .parse()
        .expect("please give me correct string number!");
}

fn get_diff(us_guess: i16, guess: i16) -> i16 {
    return (us_guess - guess).abs();
}

fn run_game() {
    let mut rng = rand::thread_rng();
    let guess = rng.gen_range(1..=10);
    println!("\nPlayer 1, Please input your guess:");
    let pl1_guess = get_guess();

    println!("Player 2, Please input your guess:");
    let pl2_guess = get_guess();

    if get_diff(pl1_guess, guess) < get_diff(pl2_guess, guess) {
        println!("Player 1 wins!")
    } else if get_diff(pl2_guess, guess) < get_diff(pl1_guess, guess) {
        println!("Player 2 wins!")
    } else {
        println!("It's a tie!")
    }
}

fn main() {
    println!("Guessing game");
    loop {
        run_game();
        let mut can_continue = String::new();
        println!("Do you want to continue? (Y/N)");
        io::stdin()
            .read_line(&mut can_continue)
            .expect("Failed to read line");

        if can_continue.trim() == "n" || can_continue.trim() == "N" {
            println!("Thanks for playing");
            break;
        }
    }
}
