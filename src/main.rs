mod error;
mod lexer;
mod token;

use error::{ExitError, Result};
use std::{env, process};

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
    let lexer = lexer::Lexer::new(&input);
    for token in lexer {
        println!("{:?}", token)
    }
    Ok(())
}
