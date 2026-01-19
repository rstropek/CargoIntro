# Hands-on Lab

## Introduction

This is a Rust hands-on lab for beginners. You can find a sample solution [in GitHub](https://github.com/rstropek/mth-calc). However, if you want to practice, try to implement the program on your own and only look at the solution if you are stuck.

You will practice in this lab:

* Add documentation to your package
  * Including example code
* Publish create to *crates.io*

Note that this lab builds on [the previous one](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git). It assumes that are familiar with it and its learning goals. If you do not want to invent documentation, you can copy sample text from [*creates.io*](https://crates.io/crates/mth_calc).

## The Requirements

* Ensure that the library that you created in [the previous exercise](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git) fulfills all [requirements](https://doc.rust-lang.org/cargo/reference/publishing.html#before-publishing-a-new-crate) for being published to *crates.io*. This includes in particular:
  * Meaningful metadata in *Cargo.toml*
  * Add *README.md*

* Document your API properly
  * Add at least one example

* Publish your library to *creates.io*

* Modify the [the previous exercise](https://github.com/rstropek/CargoIntro/tree/master/samples/12-crates-deps-git) so that it uses your published create from *crates.io*
## Non-Functional Requirements

* Make sure your library compiles, all unit tests succeed, and the example code is working
