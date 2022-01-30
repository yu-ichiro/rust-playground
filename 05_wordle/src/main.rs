mod wordle;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use colored::*;


enum WordleStatus {
    Unknown,
    Missed,
    Used,
    Hit,
}

fn format_letter(letter: &char, status: &WordleStatus) -> ColoredString {
    match status {
        WordleStatus::Unknown => letter.to_string().as_str().normal(),
        WordleStatus::Missed => letter.to_string().as_str().black(),
        WordleStatus::Used => letter.to_string().as_str().yellow(),
        WordleStatus::Hit => letter.to_string().as_str().underline().green(),
    }
}

fn print_letters(letters: &HashMap<char, WordleStatus>) {
    for c in 'A'..='Z' {
        print!("{}", format_letter(&c, letters.get(&c).unwrap()));
    }
    println!();
}

fn main() {
    let words = wordle::words();
    println!("Loaded {} words", words.len());
    let target = words.iter()
        .nth(rand::thread_rng().gen_range(0..words.len()))
        .unwrap().clone()
        .to_uppercase();

    let chances: u8 = 6;
    let mut guesses: u8 = 0;

    let mut letters: HashMap<char, WordleStatus> = ('A'..='Z')
        .map(|c| (c, WordleStatus::Unknown))
        .collect();

    loop {
        print_letters(&letters);
        print!("Enter your guess ({}/{}): ", guesses + 1, chances);
        io::stdout().flush().expect("could not flush stdout");
        let mut guess = String::new();

        let guess = match io::stdin()
            .read_line(&mut guess)
            .ok()
            .and_then(|_| {
                words.get(guess.to_lowercase().trim())
            }) {
            None => {
                println!("No such word in the dictionary. try a different one.");
                continue;
            }
            Some(_) => guess.trim().to_uppercase()
        };

        let mut guess_to_target: HashMap<usize, Option<usize>> = HashMap::from([
            (0, None),
            (1, None),
            (2, None),
            (3, None),
            (4, None),
        ]);

        let mut target_to_guess: HashMap<usize, Option<usize>> = HashMap::from([
            (0, None),
            (1, None),
            (2, None),
            (3, None),
            (4, None),
        ]);

        for (i, target_letter) in target.chars().enumerate() {
            for (j, guess_letter) in guess.chars().enumerate() {
                if guess_to_target.get(&j).unwrap().eq(&Some(j)) {
                    continue
                } else if i == j && target_letter == guess_letter {
                    guess_to_target.insert(j, Some(i));
                    target_to_guess.insert(i, Some(j));
                    letters.insert(target_letter, WordleStatus::Hit);
                    break
                } else if target_letter == guess_letter && (
                    match target_to_guess.get(&i).unwrap() {
                        None => true,
                        Some(k) => *k != i && *k > j
                    }
                ) {
                    guess_to_target.insert(j, Some(i));
                    target_to_guess.insert(i, Some(j));
                    letters.insert(target_letter, WordleStatus::Used);
                } else {
                    match letters.get(&guess_letter).unwrap() {
                        WordleStatus::Hit => {},
                        WordleStatus::Used => {},
                        _ => {
                            letters.insert(guess_letter, WordleStatus::Missed);
                        }
                    }
                }
            }
        }

        for (i, letter) in guess.chars().enumerate() {
            sleep(Duration::from_millis(500));
            print!("{}", match guess_to_target.get(&i).unwrap() {
                None => format_letter(&letter, &WordleStatus::Unknown),
                Some(k) if *k == i => format_letter(&letter, &WordleStatus::Hit),
                Some(_) => format_letter(&letter, &WordleStatus::Used),
            });
            io::stdout().flush().expect("could not flush stdout");
        }
        println!();

        if guess_to_target.iter().all(|(k, item)| item.eq(&Some(*k))) {
            println!("You win! Tries: {}", guesses + 1);
            break;
        }

        guesses += 1;
        if guesses == chances {
            println!("Game over: {}", target);
            exit(1);
        }
    }
}
