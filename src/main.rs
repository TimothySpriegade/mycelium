use std::env;
use std::io;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;
pub mod types;

fn main() {
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| String::from("Developer"));

    println!(
        "Hello {}! This is the mycelium programming language!",
        username
    );
    println!("Have fun trying some commands");

    let stdin = io::stdin();
    let mut in_stream = stdin.lock();

    let stdout = io::stdout();
    let mut out_stream = stdout.lock();

    if let Err(e) = repl::repl::start(&mut in_stream, &mut out_stream) {
        eprintln!("Error running REPL: {}", e);
        std::process::exit(1);
    }
}
