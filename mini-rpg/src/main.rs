use std::io;
use rand::Rng;

fn main() {
    let mut inventory: Vec<String> = Vec::new(); // created an empty array called inventory to store things in it (we can use this variable in all the matches in main)
    loop {
        println!("\n Hello and welcome to my mini-rpg project, this is an experimental mini-game to in the future make more\n\n Do you want to play? yes/no");
        let mut play = String::new();
        io::stdin()
            .read_line(&mut play)
            .expect("Error reading the input or bad input");
        let play = play.trim();

        match play {
            "yes" => {
                println!("Welcome, you are an elf lost in a forest, you lost all your inventory and you are so confused \n The forest is on fire so, what do you want to do? \n\n Run away => command: run \n Stay and search for utils => command: stay "); // intro
                let mut response1 = String::new();
                io::stdin()
                    .read_line(&mut response1)
                    .expect("Incorrect input");
                let response1 = response1.trim();
                match response1 {
                    "run" => {
                        let  chance: u8 = rand::thread_rng().gen_range(1..=10);
                        // Maybe change the dice condition cause too dificult
                        println!("\nYou decided to run but unfortunetly you found a trap and now your life is at risk \n\n now we are going to throw a dice from 0 to 10, if the result is more or equal to 5 you will live \n but if it's less, you will die");
                        if chance >= 5 {
                            println!("Wow!, you maginifcally solved the problem and didn't die in the trap!, dice: {}", chance);
                        } else {
                            println!("The god's aren't with you this time, YOU DIED, dice: {}", chance);
                            break;
                        }

                        println!("Now that you passed that trap, you feel a bit hungry but you don't have any food\n\n Do you want to search for some food?\n\n yes => command: yes\n no => command: no");
                        let mut response2 = String::new();
                        io::stdin()
                            .read_line(&mut response2)
                            .expect("Incorrect input");
                        let response2 = response2.trim();
                        match response2 {
                            "yes" => {
                                println!("You search for some food in the forest but the smoke make difficult the search, by the way you find some mushrooms\n\n +mushrooms");
                                inventory.push("Mushrooms".to_string()); // we put in the array inventory mushroooms
                                println!("Your inventory: {:?}", inventory);
                                println!("Now the smoke is more annoying, and a strange sound it's starting to come behind you, what do you want to do?\n\n look behind => command: behind\n RUN => command: run");
                                let mut response3 = String::new();
                                io::stdin()
                                    .read_line(&mut response3)
                                    .expect("Error in your input");
                                let response3 = response3.trim();
                                match response3 {
                                    "behind" => {
                                        let chance1: u8 = rand::thread_rng().gen_range(16..=20); // hehehehe :))))
                                        println!("You decide to look back and you found 6 dark elfs surrounding you, they don't seem friendly.\n\n Your life is at risk, a dice from 1 to 20 will be thrown and if the result is equal or more than 15 you will survive\n\n Alea iacta est...");
                                        if chance1 >= 15 {
                                            println!("You escaped succesfuly! incredible dice: {}", chance1);
                                        } else {
                                            println!("You tried to escape and... you were too solow, the killed you :( dice: {}", chance1);
                                            break;
                                        }
                                    }

                                    "run" => {
                                        println!("You decided to escape") // continue here
                                    }

                                    _=> {
                                        println!("Invalid input")
                                    }
                                }

                            }
                            //////////////////////////////////////////////////
                            "no" => {

                            }
                            _=>{
                                println!("Invalid input")
                            }
                        }

                    }

                    "stay" => {
                        // what happens if the elf stays and don't runs away

                    }

                    _=> {
                        println!("Invalid input")
                    }
                }
            }

            "no" => {
                println!("Okay, have a nice day!");
                break;
            }

            _=> {
                println!("invalid input") // put suggested input?
            }
        }
    }
}
