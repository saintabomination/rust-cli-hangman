use std::io;
use std::io::Write;

fn print_word(word: &String) -> () {
    let character_count = word.chars().count();
    println!("{} ({})", "_".repeat(character_count), word);
}

fn check_guess(word: &String, guessed_letter: &char, shown_letters: &mut Vec<char>) -> () {
    if !word.contains(*guessed_letter) {
        return;
    };
    if !shown_letters.contains(guessed_letter) {
        shown_letters.push(*guessed_letter);
    }
    println!("{:?}", shown_letters);
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

        print_word(&my_word);
        check_guess(&my_word, &letter.trim().chars().next().unwrap(), &mut shown_letters);
    }
}
