use std::collections::hash_map::Entry;
use std::fs;
use std::io;
use std::collections::HashMap;
use std::iter;
use std::ops;
use rand;

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn count_characters(string: &str) -> HashMap<char, i32> {
    let mut character_count = HashMap::new();
    for character in string.chars() {
        character_count.entry(character)
            .and_modify(|count| *count += 1 )
            .or_insert(1);
    }
    character_count
}

fn colour_guess(answer: &str, guess: &str) -> String {
    let mut answer_character_count = count_characters(answer);
    let mut colored_guess = String::new();

    for (guess_character, answer_character) in iter::zip(guess.chars(), answer.chars()) {
        let colour_reset = "\x1b[38;5;15m";
        match &mut answer_character_count.entry(guess_character) {

            Entry::Occupied(occupied_entry) => {
                let count = occupied_entry.insert(occupied_entry.get() - 1);
                let colour_modifier =
                    if count <= 0 { "8" } // character matches to no unused character in the answer
                    else if guess_character != answer_character { "3" } // character is correct, but in the wrong place
                    else { "2" }; // character is correct and in the right place
                colored_guess += &format!("\x1b[38;5;{colour_modifier}m{guess_character}{colour_reset}");
            }
            Entry::Vacant(_) => {
                colored_guess += &format!("\x1b[38;5;8m{guess_character}{colour_reset}");
            }
        }
    }
    colored_guess
}

fn print_game_state(answer: &str, guesses: &Vec<String>) -> () {
    for guess in guesses {
        println!("{}", colour_guess(answer, guess));
    }
}

const WORD_RANGE: ops::RangeInclusive<usize> = 500..=1000;

fn main() {
    let words = read_lines("data/words.txt");
    let answer = &words[rand::random_range(WORD_RANGE)];

    let mut guesses = Vec::new();
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .unwrap();
        guess = guess
            .trim()
            .to_string();
        if words.iter().any(|word| word == &guess) {
            guesses.push(guess.clone());
            print_game_state(answer, &guesses);
            println!();
        }
        if guess == *answer { break; }
    }
    println!("Well done, you got the word in {} guesses!", guesses.len());
}
