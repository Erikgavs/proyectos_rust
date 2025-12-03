use std::io;

fn main() {
    println!("Put a word to count the letters of the word\n");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Error reading input");
    let word = word.trim();
    let mut count = 0;

    for _ in word.chars(){ // _ => We put this because we don't need to use the variable
       count += 1;
    }

    println!("Your word: {} have {} letters", word, count)

}
