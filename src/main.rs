use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess (between 1 and 10)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let message = String::from("You win! :)");
                let width = message.chars().count();
                let mut writer = BufWriter::new(stdout.lock());
                say(message.as_bytes(), width, &mut writer).unwrap();
                break;
            }
        }
    }
}
