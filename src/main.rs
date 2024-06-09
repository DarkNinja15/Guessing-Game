use std::io;
fn main() {
    println!("Guess the number!");

    println!("Enter your number");

    let mut guess=String::new();

    io::stdin().read_line( &mut guess).expect("Failed to read the input");


    println!("You guessed: {guess}");
}