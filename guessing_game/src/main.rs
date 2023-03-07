use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Enter a number : ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed read line");

    println!("You entered {guess} and secret is {secret_number}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Match !")
    }
}
