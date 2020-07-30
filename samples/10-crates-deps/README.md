# Hands-on Lab

## Introduction

This is a Rust hands-on lab for beginners. You can find the sample solution in this folder. However, if you want to practice, try to implement the program on your own and only look at the solution if you are stuck.

You will practice in this lab:

* Working with *crates.io* dependencies
* Various Rust language fundamentals
* Basics about working with threads
* Basics about generating random numbers

## The Requirements

* Write a program that calculates PI by approximation. The program should [use the Monte Carlo method for that](https://academo.org/demos/estimating-pi-monte-carlo/).

* The number of iterations can be a fixed constant value in your code.
  * Want an extra challenge: Specify the number of seconds the calculation should last instead of a fixed number of iterations.
  * Want an even harder challenge: Calculate until the user hits a key. While calculating, continuously print the current PI estimation on the screen.

* At the end of the program, print the average number of evaluated points per second.
 
## Non-Functional Requirements

* Implement this program with the latest version of Rust.

* Execute the iterations **in parallel on all available CPUs**. 

* Compare running your program with *debug* and with *release* configuration.

## Sample Output

```txt
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `...`
3.1422625
9,185,097 calculations per second
$
```
