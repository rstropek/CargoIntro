# Cargo

## The Rust Package Manager

Rainer Stropek | @rstropek | Coding Club Linz

----

# 5 Minute Jumpstart

For Beginners

---

## Purpose

* Dependency <!-- .element: class="fragment" --> management
* Invoke <!-- .element: class="fragment" --> Rust compiler

```bash
cargo new hello_world --bin # --bin for program, --lib for library

cd hello_world/
cargo build
./target/debug/hello_world

cargo build --release
./target/release/hello_world
```
<!-- .element: class="fragment" -->

---

## *Cargo.toml*

* Similar <!-- .element: class="fragment" --> to npm's *package.json*
* Has <!-- .element: class="fragment" --> to be called *Cargo.toml* (uppercase *C*)
  * [*Tom's Obvious, Minimal Language*, short *TOML*](https://github.com/toml-lang/toml)
* Built <!-- .element: class="fragment" --> on [Semantic Versioning](https://semver.org/)
* Package  <!-- .element: class="fragment" --> metadata (more about the [Manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html)):

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Rainer Stropek <rainer@software-architects.at>"]
edition = "2018" # Rust edition (default is 2015, current is 2018)

[dependencies]
# No dependencies yet, will follow in a minute
```
<!-- .element: class="fragment" -->

---

## [Sample](https://github.com/rstropek/CargoIntro/tree/master/samples/01-intro)

```toml
[dependencies]
regex = "1"
lazy_static = "1"
```

```rust
use std::io;

// We can now start using our dependencies in code
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Type 'quit' to exit.");

    let mut input = String::new();
    loop {
        // Let user enter some text and remove trailing \n
        io::stdin().read_line(&mut input).unwrap();
        input.truncate(input.len() - 1);

        // Say goodbye if user want to quite
        if input == "quit" {
            println!("Good bye");
            break;
        }

        // Check if input is a date and print answer
        let is_date = is_proper_date(&input);
        println!("{0} is{1}a date", input, if is_date { " " } else { " not " });

        // Clearn input buffer and start all over
        input.clear();
    }
}

/// Indicates whether a given text contains a properly formated date string.
/// The recognized date format is YYYY-MM-DD.
fn is_proper_date(text: &str) -> bool {
    // It makes no sense to recompile regex every time we run the function.
    // So lets use a lazy evalulated static instead.
    lazy_static! {
        static ref RE: Regex = Regex::new("^(\\d{4})-(\\d{2})-(\\d{2})$").unwrap();
    }
    
    RE.is_match(text)
}
```

---

## [Project File Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)

```txt
.
├── Cargo.lock
├── Cargo.toml
├── src/ 
│     Source code
├── benches/
│     Benchmarks
├── examples/
│     Example code
└── tests/ (integration tests)
      Integration tests
```

----

# Dependencies

How to specify dependencies

---

## Sources
| Source                          | Description |
| ------------------------------- | ----------- |
| Version                         | [crates.io](https://crates.io/) or [custom registry](https://doc.rust-lang.org/cargo/reference/registries.html#using-an-alternate-registry) (e.g. [Cloudsmith](https://cloudsmith.com/blog/worlds-first-private-cargo-registry-w-cloudsmith-rust/), [GitHub](https://doc.rust-lang.org/cargo/reference/registries.html#running-a-registry)) |
| `git`                           | Pull git repo and look for crate there |
| `path`                          | Look for create in local folder |
| Mulitple sources                | Combine registry version and `git` or `path` |

---

## [Referencing *crates.io*](https://github.com/rstropek/CargoIntro/tree/master/samples/10-crates-deps/Cargo.toml)

```toml [7-10]
[package]
name = "crates_deps"
version = "0.1.0"
authors = ["Rainer Stropek <rainer@software-architects.at>"]
edition = "2018"

[dependencies]
rand = "0.7"
num_cpus = "1.0"
num-format = "0.4"
```
