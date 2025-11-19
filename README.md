# 🦀 Rust Projects
This repository is intended to save my firsts projects with rust and use it in future projects

This repository its also intended to allow begginers to practice rust with small projects.


## 📖 Projects

### 👋 Greeter

This mini project is designed to reply to three commands "hello", "number", "bye".
When we run one of these commands, with a match we execute different actions.


[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/saludador)

🧠 **Used rust elements**

| Class / Category        | Rust Element                      |
| ----------------------- | --------------------------------- |
| Imports                 | `use std::io`                     |
| Macros                  | `println!`                        |
| Variables               | `let, let mut`                    |
| Strings                 | `String::new()`                   |
| User input              | `io::stdin().read_line(&mut var)` |
| String methods          | `trim()`                          |
| String methods          | `split_whitespace()`              |
| Vectors                 | `Vec<&str>`                       |
| Vector methods          | `.collect()`                      |
| Vector methods          | `.last()`                         |
| Control flow            | `loop`                            |
| Control flow            | `match`                           |
| Types / parsing         | `parse::<f32>()`                  |
| Strings / chars         | `.chars().next().unwrap()`        |
| Arithmetic operators    | `+ - * /`                         |
| Control flow            | `continue`                        |
| Control flow            | `break`                           |
| Errors / basic handling | `.expect("message")`              |



### ♦️ Random Number

This project is designed to ask a numbrer between 1 and 10 (both included), your mission is to match the number with the script

[Link to the project](https://github.com/Erikgavs/proyectos_rust/tree/main/random_num)

🧠 **Used rust elements**
| Rust Element                            | Type / Category             |
| --------------------------------------- | --------------------------- |
| `use rand::Rng;`                        | `use` declaration           |
| `use std::io;`                          | `use` declaration           |
| `fn main() { ... }`                     | Main function               |
| `let mut rng = rand::thread_rng();`     | Mutable variable            |
| `let numero = rng.gen_range(1..=10);`   | Variable                    |
| `loop { ... }`                          | Infinite loop               |
| `println!(...)`                         | Print macro                 |
| `let mut respuesta = String::new();`    | Mutable variable            |
| `io::stdin().read_line(&mut respuesta)` | Standard input              |
| `respuesta.trim().parse().expect(...)`  | Conversion / Error handling |
| `if ... { ... } else { ... }`           | Conditional                 |
| `break`                                 | Loop exit                   |




## 🧙‍♂️ Authors 

- [@Erikgavs](https://www.github.com/Erikgavs)
