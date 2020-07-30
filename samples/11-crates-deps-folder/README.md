# Hands-on Lab

## Introduction

This is a Rust hands-on lab for beginners. You can find the sample solution in this folder. However, if you want to practice, try to implement the program on your own and only look at the solution if you are stuck.

You will practice in this lab:

* Working with *path dependencies*
* Various Rust language fundamentals
* Basics about creating Rust libraries
* Basics about unit testing

Note that this lab builds on [the previous one](https://github.com/rstropek/CargoIntro/tree/master/samples/10-crates-deps). It assumes that are familiar with it and its learning goals.

## The Requirements

* Extract the logic of running a given closure (aka lambda function) in parallel on all given CPUs in a function in a separate library called *mth-calc* (for *multi-threaded calculation*).

* Design your library so that it can be used for our *PI Monte Carlo* problem.

* Change your main program (*PI Monte Carlo*) so that it uses the library. The goal is that the main program does not include and threading-logic anymore. All the threading logic has to be encapsulated in the library.

## Non-Functional Requirements

* Add at least one meaningful unit test to your library.

## Sample Output

The output of your new application should not changed compared to [the previous version](https://github.com/rstropek/CargoIntro/tree/master/samples/10-crates-deps).
