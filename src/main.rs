use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your number");
        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You win!!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too Big!!"),
            std::cmp::Ordering::Less => println!("Too Small!!"),
        }
    }
}
