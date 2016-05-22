extern crate ascii_clock;

use std::io;
use std::io::Write;
use ascii_clock::generator::generate_clock;
use ascii_clock::time::Time;

fn main() {
    print!("Please enter the time you would like to see (in hh:mm format, 24 hour time OK): ");
    io::stdout().flush().unwrap();

    let mut raw_input = String::new();

    if let Err(error) = io::stdin().read_line(&mut raw_input) {
        println!("Couldn't read input: {}", error);
        return;
    }

    let trimmed_input = raw_input.trim();

    match Time::parse(trimmed_input) {
        Some(time) => {
            println!("{}", generate_clock(time));
        }
        None => println!("Unrecognised input format. Please try again."),
    }
}
