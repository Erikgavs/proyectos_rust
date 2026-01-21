# 🦀 Rust Projects

This repository is intended to save my first projects with Rust and use them in future projects.

This repository is also intended to allow beginners to practice Rust with small projects.


# 📖 Projects

## 🔢 Word Count

A simple yet fundamental application that demonstrates character iteration in Rust. The program takes a word as input and counts its letters using a for loop, providing hands-on experience with basic iteration patterns.

**Key Concepts Demonstrated:**
- User input handling with `std::io`
- String trimming and processing
- For loops with iterator pattern
- Character iteration with `.chars()`
- Mutable counters and incrementing
- Unused variable notation with `_`

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/word_count)

---

## 🫸 Word Splitter

An interactive application that splits phrases into individual words using Rust's string manipulation methods. The program demonstrates the power of the `split_whitespace()` method and allows users to try multiple phrases in a single session.

**Key Concepts Demonstrated:**
- String splitting with `split_whitespace()`
- Vector creation from iterators with `.collect()`
- Match expressions for flow control
- Loop continuation and breaking
- User session management

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/word_splitter)

---

## 🔄 Reverter

A versatile string manipulation tool that can reverse both individual words and entire phrases. The application showcases different reversal techniques: character-level reversal for words and word-order reversal for phrases, demonstrating the flexibility of Rust's string handling capabilities.

**Key Concepts Demonstrated:**
- Character reversal with `.chars().rev().collect()`
- Vector manipulation with `.reverse()`
- String joining from vectors with `.join()`
- Pattern matching for user options
- Conditional program flow with `if-else`
- Loop control with `continue` and `break`

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/reverter)

---

## ♦️ Random Number

A number guessing game that challenges users to match a randomly generated number between 1 and 10. This project introduces external crate usage (`rand`) and demonstrates the fundamental game loop pattern commonly used in interactive applications.

**Key Concepts Demonstrated:**
- External crate integration (`rand`)
- Random number generation with `gen_range()`
- Type parsing with `parse()` and error handling
- Infinite game loops
- Comparison operators and conditionals

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/random_num)

---

## 👋 Greeter

An interactive command-line application that responds to three different commands: "hola" (greeting with name extraction), "numero" (basic calculator), and "adios" (exit). This comprehensive project combines multiple Rust concepts into a single application, serving as an excellent learning resource for beginners.

**Key Concepts Demonstrated:**
- Complex match expressions with multiple arms
- Nested user input handling
- String parsing and extraction with `split_whitespace()`
- Type conversion with `parse::<f32>()`
- Character extraction with `.chars().next().unwrap()`
- Arithmetic operations on parsed numbers
- Nested match expressions for calculator logic
- Vector manipulation with `.last()` method

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/greeter)

---

## 🧙‍♂️ Mini-RPG

A text-based role-playing game where you play as an elf lost in a burning forest. This ambitious project features branching narratives, dice-roll mechanics, inventory management, and multiple decision points that affect the outcome. The game demonstrates complex control flow and state management in Rust.

**Key Concepts Demonstrated:**
- Deeply nested match expressions for narrative branching
- Vector-based inventory system with `.push()` for dynamic item management
- Random number generation for dice roll mechanics
- Thread sleeping with `Duration::from_secs()` for dramatic timing
- Multi-state management across game progression
- Complex conditional logic with probability-based outcomes
- Modular code organization with separate modules

[Link to project](https://github.com/Erikgavs/proyectos_rust/tree/main/normal_projects/mini-rpg)

---

## 🧙‍♂️ Author

- [@Erikgavs](https://www.github.com/Erikgavs)
