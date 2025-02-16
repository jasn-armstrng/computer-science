#### A Lox Interpreter implementation in Rust

In the course of implementing a Lox interpreter in Rust, we will encounter several challenges and opportunities for learning. Here's a high-level overview of the tasks ahead:

 - [X] Read a source file
 - [ ] Tokenize the source file
 - [ ] Parse the tokens to an AST
 - [ ] Evaluate the expressions on the AST
 - [ ] Print the result of the evaluation

Each of the above is a combination of sub-problems of different complexity that need to be understood in themselves and how they contribute to the wider system.

#### Section 1: Read a source file
Section input(s):
 1. Source file. This is the .lox file that contains the Lox code to be interpreted. It should not be altered during the execution of the interpreter.
Input flow: CLI -> main() -> run() -> read_source()

Section output(s):
 1. Error from main() if no file argument provided
 2. Error from read_source() if, when attempting to open the file
    - File not found
    - File is protected
    - Some other error
    or when attempting to read the file
    - File is not a valid UTF-8 encoded text file
    - Some other error reading
Error flow: read_source() -> run() -> main() -> CLI
