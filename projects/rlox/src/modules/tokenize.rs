use std::iter::Peekable;
use std::str::Chars;
/// Scanning: Chapter 4 of "Crafting Interpreters"
// [Note] The tokenizer (scanner) takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
// [Note] Tokens are the meaningful "words" and "symbols" that make up the language's grammar.

/// Enum representing the different types of tokens that can be produced by the tokenizer.
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Illegal,

    EOF,
}

/// Struct representing a token produced by the tokenizer.
/// Tokens produced by the tokenizer have a type, a lexeme (the actual text of the token), and a line number.
#[derive(Debug)] // [Note] Derive must be implemented for enclosed structs. Here, TokenType
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: usize,
}

impl Token {
    // Token constructor
    pub fn new(token_type: TokenType, lexeme: String, line: u32, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}

/// The tokenizer machine responsible for converting source code into tokens.
pub struct Tokenizer<'a> {
    // source: &'a String,  // Don't need to store the source string anymore
    // [Note] 'a is a lifetime annotation.  It indicates that the
    // Tokenizer struct cannot outlive the String it borrows.
    // In other words, the Tokenizer's reference to the String
    // must be valid for at least as long as the Tokenizer itself exists.

    // State variables
    chars: Peekable<Chars<'a>>,
    start: usize,
    current: usize, // [Question] Why is usize used here for the current position in the source code?
    // [Answer] ...
    line: u32, // [Question] What is the difference between a usize and a u32?
               // [Answer] usize is an unsigned integer type that can hold the maximum value of the system's pointer size.
               // u32 is an unsigned integer type that can hold values from 0 to 4294967295.
}

impl<'a> Tokenizer<'a> {
    // [Note] The 'a here mirrors the lifetime of the struct,
    // ensuring consistency.  All methods of Tokenizer
    // will be constrained by this lifetime.

    // Tokenizer constructor
    pub fn new(source: &'a String) -> Self {
        // [Note] The input source String also has
        // the lifetime 'a, tying it to the
        // Tokenizer's lifetime.
        Tokenizer {
            chars: source.chars().peekable(), // Create a peekable iterator handler for the source
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Main entry point for scanning tokens
    //  [Note] scan_tokens does not move the iterator forward
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // Create a peekable iterator handler for the source
        // let mut chars = self.source.chars().peekable();

        // Collection for scanned tokens
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(&c) = self.chars.peek() {
            // Scanning here
            self.start = self.current;
            let token = self.scan_token(c);
            tokens.push(token);
        }

        // When scanning is complete append the EOF token to tokens
        tokens.push(self.create_token(TokenType::EOF, String::from("")));
        // Check tokens
        dbg!(tokens)
    }

    pub fn create_token(&mut self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line, self.current)
    }

    // [Note] This function does the below,
    // 1. Classifies and creates a token
    // 2. Each of its sub-routines move the iterator forward.
    pub fn scan_token(&mut self, c: char) -> Token {
        self.skip_whitespace_tabs_newlines();

        self.match_single_and_operator_tokens(&c)
    }

    pub fn skip_whitespace_tabs_newlines(&mut self) {
        match self.chars.peek() {
            Some(&' ') => {
                // TODO: Refactor, combine the below into an advance.
                self.chars.next();
                self.current += 1;
            }
            Some(&'\t') => {
                self.chars.next();
                self.current += 1;
            }
            Some(&'\r') => {
                self.chars.next();
                self.current += 1;
            }
            Some(&'\n') => {
                // Increment line number and consume the newline character
                // [Note] Allows the tokenizer to correctly track line number positions within the source code.
                self.line += 1;
                self.current = 0;
                self.chars.next();
            }
            _ => {}
        }
    }

    // [Question] Why is there a lifetime partnership between the function and the char reference.
    // [Answer] The lifetime partnership is necessary because the function needs to borrow the char reference for the duration of the function call.
    // This ensures that the char reference remains valid throughout the function call.
    pub fn match_single_and_operator_tokens<'b>(&mut self, c: &'b char) -> Token {
        match c {
            '(' => {
                self.chars.next(); // consume the character
                self.current += 1;
                self.create_token(TokenType::LeftParen, c.to_string())
            }
            ')' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::RightParen, c.to_string())
            }
            '{' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::LeftBrace, c.to_string())
            }
            '}' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::RightBrace, c.to_string())
            }
            '.' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Dot, c.to_string())
            }
            ',' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Comma, c.to_string())
            }
            ';' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Semicolon, c.to_string())
            }
            '=' => {
                // [Question] In what case would unwrap() panic!
                // [Answer] If the next character is not available, unwrap() will panic!
                // [Note] The next character is always available because we peeked it before in scan_tokens()
                // Still favor unwrap_or()
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::EqualEqual, c.to_string())
                } else {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::Equal, c.to_string())
                }
            }
            '/' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Slash, c.to_string())
            }
            '*' => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Star, c.to_string())
            }
            '!' => {
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::BangEqual, c.to_string())
                } else {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::Bang, c.to_string())
                }
            }
            '<' => {
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::LessEqual, c.to_string())
                } else {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::Less, c.to_string())
                }
            }
            '>' => {
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::GreaterEqual, c.to_string())
                } else {
                    self.chars.next();
                    self.current += 1;
                    self.create_token(TokenType::Greater, c.to_string())
                }
            }
            _ => {
                self.chars.next();
                self.current += 1;
                self.create_token(TokenType::Illegal, c.to_string())
            }
        }
    }
}
