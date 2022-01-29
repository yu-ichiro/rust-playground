use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process::exit;
use rand::Rng;

fn main() {
    let target: u32 = rand::thread_rng().gen_range(1..=100);
    let chances: u32 = 10;
    let mut tries: u32 = 0;

    loop {
        print!("Enter your guess (1~100): ");
        io::stdout().flush().expect("failed flushing stdout");

        let mut guess = String::new();
        let guess: Option<u32> = io::stdin()
            .read_line(&mut guess).ok()
            .and_then(|_| {
                guess.trim().parse().ok()
            });
        println!("{}", guess.is_some());
        if guess.is_none() {
            println!("Failed to parse the guess. Try again.");
            continue
        }
        let guess = guess.unwrap();

        let result = match guess.cmp(&target) {
            Ordering::Greater => "Too high!",
            Ordering::Less => "Too low!",
            Ordering::Equal => {
                println!("You're correct! Number of tries: {}", tries + 1);
                return;
            },
        };

        tries += 1;
        if tries < chances {
            println!("{} Number of chances left: {}", result, chances - tries);
        } else {
            println!("{} No more chances left. The answer was: {}", result, target);
            exit(1);
        }

    }
}
