use std::io;

fn main() {
    println!(" \n \n Hello, this script can revert the phrases or words that you pass in it \n \n Do you want to pass a phrase or a word? \n word/phrase");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Error reading the input");

    let response = response.trim();
    loop {
        match response {
            "word" => {
                println!("Put a word and the program will reverse it");
                let mut word = String::new();
                io::stdin()
                    .read_line(&mut word)
                    .expect("Error reading the input");
                let word = word.trim();
                let reverse: String = word.chars().rev().collect();
                // word.chars() => this makes 't', 'e', 'x', 't' |c| (each letter is called now c), every c (letter) we convert it to string
                println!("Look at your word! {} \n", reverse);

                println!("Do you want to try again? yes/no");
                let mut again = String::new();
                io::stdin()
                    .read_line(&mut again)
                    .expect("Error reading input");
                let again = again.trim();

                if again == "yes" {
                    continue;
                } else {
                    break;
                }
            }

            "phrase" => {
                println!("Put a phrase and the program will reverse it");
                let mut phrase = String::new();
                io::stdin()
                    .read_line(&mut phrase)
                    .expect("Error reading the input");
                let phrase = phrase.trim();
                let mut split: Vec<&str> = phrase.split_whitespace().collect(); // => phrase= "...", "...", "..."
                split.reverse();
                let reversed = split.join(" ");

                println!("Here's your phrase reversed: {} \n", reversed);

                println!("Do you want to try again? yes/no");
                let mut response_user = String::new();
                io::stdin()
                    .read_line(&mut response_user)
                    .expect("Error reading the input");
                let response_user = response_user.trim();
                if response_user == "yes"{
                    continue;
                } else{
                    break;
                }
            }

            _=> {
                println!("invalid input");
            }
        }
    }
}
