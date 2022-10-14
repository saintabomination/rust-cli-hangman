use std::io;
use std::io::Write;

fn get_current_word_state(word: &String, shown_letters: &Vec<char>) -> String {
    let mut output = String::new();
    for character in word.chars() {
        if shown_letters.contains(&character) {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    return output;
}

fn check_guess(word: &String, guessed_letter: &char, shown_letters: &mut Vec<char>) -> () {
    if !word.contains(*guessed_letter) {
        return;
    };
    if !shown_letters.contains(guessed_letter) {
        shown_letters.push(*guessed_letter);
    }
}

fn check_win(word_state: &String) -> bool {
    return !word_state.contains("_");
}

fn main() {
    println!("Hangman CLI");
    let my_word = String::from("abacus");
    let mut shown_letters: Vec<char> = Vec::new();
    let mut total_guesses: u8 = 0;

    println!("\n{}\n", get_current_word_state(&my_word, &shown_letters));

    loop {
        print!("Enter a letter: ");
        io::stdout().flush().expect("Flush failed!");

        let mut letter = String::new();
        io::stdin()
            .read_line(&mut letter)
            .expect("Your input is not valid.");

        total_guesses += 1;
        check_guess(&my_word, &letter.trim().chars().next().unwrap(), &mut shown_letters);
        println!("\n{}\n", get_current_word_state(&my_word, &shown_letters));
        println!("Total guesses: {}", total_guesses);

        if check_win(&get_current_word_state(&my_word, &shown_letters)) {
            break;
        }
    }

    println!("You won.");
}
