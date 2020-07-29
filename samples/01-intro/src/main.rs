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
