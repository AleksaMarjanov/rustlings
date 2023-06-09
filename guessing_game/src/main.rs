use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number from 0 to 100");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

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
            Ordering::Less => println!("Too small! Try a bigger number"),
            Ordering::Greater => println!("Too Big! Try a smaller number"),
            Ordering::Equal => {
                println!("Wise choice!! You won...");
                break;
            }
        }
    }
}
