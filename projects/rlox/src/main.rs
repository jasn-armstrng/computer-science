// External modules
pub mod modules;
use crate::modules::reader::*;

// Standard libarary
use ::std::env;
use std::process;

// Interpreter entry point
fn run(args: &[String]) {
    // NOTE: We expect that that args[1] is the filename. All other arguments entered at the command-line are ignored
    //
    read_source(&args[1]);
}

fn main() {
    // Collect arguments from the command-line
    let args: Vec<String> = env::args().collect(); // Note: This will panic if the argument contains invalid Unicode

    // Check if an argument was provided
    if args.len() < 2 {
        eprintln!("ERROR: Not enough arguments");
        process::exit(1);
    }

    // Enter intepreter main
    run(&args);

    dbg!(args);
}
