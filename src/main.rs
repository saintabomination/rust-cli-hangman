use std::io;
use std::io::Write;

fn print_word(word: &String, shown_letters: &Vec<char>) -> () {
    let mut output = String::new();
    for character in word.chars() {
        if shown_letters.contains(&character) {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    println!("{}", output);
}

fn check_guess(word: &String, guessed_letter: &char, shown_letters: &mut Vec<char>) -> () {
    if !word.contains(*guessed_letter) {
        return;
    };
    if !shown_letters.contains(guessed_letter) {
        shown_letters.push(*guessed_letter);
    }
}

fn main() {
    println!("Hangman CLI\n");
    let my_word = String::from("abacus");
    let mut shown_letters: Vec<char> = Vec::new();

    loop {
        print!("Enter a letter: ");
        io::stdout().flush().expect("Flush failed!");

        let mut letter = String::new();
        io::stdin()
            .read_line(&mut letter)
            .expect("Your input is not valid.");

        check_guess(&my_word, &letter.trim().chars().next().unwrap(), &mut shown_letters);
        print_word(&my_word, &shown_letters);
    }
}
