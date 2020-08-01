mod ast;
mod error;
mod tokenizer;

use error::{ExitError, Result};
use std::{env, process};
use tokenizer::Tokenizer;

fn main() {
    match try_main() {
        Err(ExitError { code, message }) => {
            eprintln!("Error: {} (exit with status {})", message, code);
            process::exit(code)
        }
        _ => {}
    }
}

fn try_main() -> Result<()> {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let tokens = Tokenizer::new(&input);
    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
}
