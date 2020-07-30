# Hands-on Lab

## Introduction

This is a Rust hands-on lab for beginners. You can find the sample solution in this folder. However, if you want to practice, try to implement the program on your own and only look at the solution if you are stuck.

You will practice in this lab:

* *Cargo* basics (creating a Rust binary program, compiling, running)
* Various Rust language fundamentals
* Basic input (*STDIN*) and output (*STDOUT*)
* Basics about Regular Expressions in Rust
* Debugging Rust

## The Requirements

* Write a command-line program that repeatedly asks the user for input through *STDIN*.
  
* If the user types *quit*, exit the program.

* Check if the user's input is a date in the format YYYY-MM-DD. Use the regular expression `^(\\d{4})-(\\d{2})-(\\d{2})$` for that.

* Print a status message stating whether the input was a date or not.

## Non-Functional Requirements

* Implement this program with the latest version of Rust.

* Use *Cargo* to build the program from scratch, compile it, and run it.

* Try to encapsulate the date check into a separate function.

* Make sure that the regular expression is only compiled once. Do not compile it each time the user enters some text.

* Make yourself familiar with how to debug Rust code in your development environment (e.g. *Visual Studio Code*). Use a search engine to find a description about how to configure your environment properly.

## Sample Output

```txt
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `...`
Type 'quit' to exit.
The quick brown fox jumps over the lazy dog
The quick brown fox jumps over the lazy dog is not a date
2020-07-30
2020-07-30 is a date
quit
Good bye
$
```
