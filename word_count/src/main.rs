use std::io;

fn main() {
    printn!("Put a word to count the letters of the word");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Error reading input");

}
