use std::io;
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess_count: u32 = 0;

    loop {
        println!("Guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess_count += 1;
        let guess: u32 = match guess.trim().parse() { // Strips and casts to u32 from str
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Final guess count: {guess_count}");
                break;
            }
        }
        println!("Guess count: {guess_count}")
    }
}
