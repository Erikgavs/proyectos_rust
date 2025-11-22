use std::io;

fn main(){
    loop {
        println!("Put a phrase that you want to split each word");
        let mut u_entry = String::new();
        io::stdin()
            .read_line(&mut u_entry)
            .expect("Error found reading user input");
        let u_entry = u_entry.trim();
        let letters: Vec<&str> = u_entry.split_whitespace().collect();
        println!("{:?}", letters);
        println!("You want to try again? yes/no");
        let mut try_again = String::new();
        io::stdin()
            .read_line(&mut try_again)
            .expect("Error found reading user input");
        let try_again = try_again.trim();
        match try_again {
            "yes" => {
                println!("Put another phrase to split the words!");
                let mut new_phrase = String::new();
                io::stdin()
                    .read_line(&mut new_phrase)
                    .expect("Error found reading user input");
                let new_phrase = new_phrase.trim();
                let splitted: Vec<&str> = new_phrase.split_whitespace().collect();
                println!("Here's your array with splitted words! {:?}", splitted);
            }
            "no" => {
                println!("Have a good day!");
                break
            }

            _=> {
                println!("Parameter not valid");
                break
            }
        }
        // {:?} -> print array
    }
}
