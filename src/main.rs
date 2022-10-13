use std::io;
use std::io::Write;

fn main() {
    println!("Hangman CLI\n");

    loop {
        print!("Enter a letter: ");
        io::stdout().flush().expect("Flush failed!");

        let mut letter = String::new();
        io::stdin()
            .read_line(&mut letter)
            .expect("Your input is not valid.");

        println!("{}", letter);
    }
}
