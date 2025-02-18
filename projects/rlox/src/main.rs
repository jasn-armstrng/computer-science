// External modules
pub mod modules;
use crate::modules::reader::*;
use crate::modules::tokenize::*;

// Standard libarary
use ::std::env;

/// Interpreter main entry point. Executes the source file reader, tokenizer, parser, and evaluator
fn run(filepath: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Read the source file contents
    let contents = read_source(filepath)?;

    // Tokenize the source code
    let mut tokenizer = Tokenizer::new(&contents);
    let _tokens = tokenizer.scan_tokens();

    // Parse the tokens into an abstract syntax tree (AST)
    // let ast = parse(&tokens)?;

    // Evaluate the AST
    // evaluate(&ast)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // [Note] Box<dyn std::error::Error> is read as "any kind of error".

    // Check if filepath argument was provided. Gracefully exit if not.
    // All other command-line arguments > 1 are ignored
    let filepath = env::args().nth(1).ok_or("ERROR: No file path provided")?;

    // Debug filepath
    dbg!(&filepath);

    // Enter intepreter main
    run(&filepath)?;

    // [Note] Ok(()) Equivalent to C's return 0.
    Ok(())
}
