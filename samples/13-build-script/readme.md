# Hands-on Lab

## Introduction

This hands-on-lab demonstrates how to use a [Rust build script](https://doc.rust-lang.org/cargo/reference/build-script-examples.html) in order to compile a native C library that is used by our Rust app. In this demo, the C library solves the [eight queens puzzle](https://en.wikipedia.org/wiki/Eight_queens_puzzle).

You will practice in this lab:

* Working with *Rust build scripts*

Note that this lab builds on [the previous one](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git). It assumes that are familiar with it and its learning goals.

## The Requirements

* Write a Rust app that calls the native library [*qps.c*](https://github.com/rstropek/CargoIntro/blob/master/samples/13-build-script/src/qps.c).
* Build the native library together with the Rust app using a [Rust build script](https://doc.rust-lang.org/cargo/reference/build-script-examples.html)

## Sample Output

The app should print the number of solutions of the [eight queens puzzle](https://en.wikipedia.org/wiki/Eight_queens_puzzle) on the screen.
