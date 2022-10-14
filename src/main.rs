use std::io;
use std::io::Write;

fn print_word(word: &String) {
    let character_count = word.chars().count();
    println!("{}", "_".repeat(character_count));
}

fn main() {
    println!("Hangman CLI\n");
    let my_word = String::from("abacus");

    loop {
        print!("Enter a letter: ");
        io::stdout().flush().expect("Flush failed!");

        let mut letter = String::new();
        io::stdin()
            .read_line(&mut letter)
            .expect("Your input is not valid.");

        print_word(&my_word);
    }
}
