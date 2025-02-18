use std::iter::Peekable;
use std::str::Chars;
/// Scanning: Chapter 4 of "Crafting Interpreters"
// [Note] The tokenizer (scanner) takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
// [Note] Tokens are the meaningful "words" and "symbols" that make up the language's grammar.

/// Enum representing the different types of tokens that can be produced by the tokenizer.
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)] // [Note] Derive must be implemented for enclosed structs. Here, TokenType
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
    column: usize,
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
            column: 0,
        }
    }

    /// Main entry point for scanning tokens
    //  [Note] scan_tokens does not move the iterator forward
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // Collection for scanned tokens
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(&c) = self.chars.peek() {
            // Scanning here
            self.start = self.current;
            let token = self.scan_token(c);
            match token {
                Some(token) => tokens.push(token),
                None => (),
            }
        }
        // When scanning is complete append the EOF token to tokens
        tokens.push(self.create_token(TokenType::EOF, String::from("")));

        for t in &tokens {
            eprintln!(" = {:?}", t);
        }
        tokens
    }

    pub fn create_token(&mut self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line, self.column)
    }

    // [Note] This function does the below,
    // 1. Classifies and creates a valid token
    // 2. Each of its sub-routines moves the iterator forward.
    pub fn scan_token(&mut self, c: char) -> Option<Token> {
        // 1. Match and skip whitespace and escape characters
        if self.skip_whitespace_and_escape_characters(&c) {
            return None;
        }
        // 2. Match and tokenize single and operator tokens
        Some(self.match_single_and_operator_tokens(&c))
    }

    pub fn skip_whitespace_and_escape_characters(&mut self, c: &char) -> bool {
        match c {
            ' ' | '\r' | '\t' => {
                self.advance();
                return true;
            }
            '\n' => {
                // [Design] Should I put the below into a separate function?
                self.chars.next();
                self.line += 1; // Move to the next line
                self.column = 0; // Reset column to 0
                self.current += 1;
                return true;
            }
            _ => false,
        }
    }

    pub fn match_single_and_operator_tokens(&mut self, c: &char) -> Token {
        match c {
            '(' => {
                self.advance();
                self.create_token(TokenType::LeftParen, c.to_string())
            }
            ')' => {
                self.advance();
                self.create_token(TokenType::RightParen, c.to_string())
            }
            '{' => {
                self.advance();
                self.create_token(TokenType::LeftBrace, c.to_string())
            }
            '}' => {
                self.advance();
                self.create_token(TokenType::RightBrace, c.to_string())
            }
            '.' => {
                self.advance();
                self.create_token(TokenType::Dot, c.to_string())
            }
            ',' => {
                self.advance();
                self.create_token(TokenType::Comma, c.to_string())
            }
            ';' => {
                self.advance();
                self.create_token(TokenType::Semicolon, c.to_string())
            }
            '+' => {
                self.advance();
                self.create_token(TokenType::Plus, c.to_string())
            }
            '-' => {
                self.advance();
                self.create_token(TokenType::Minus, c.to_string())
            }
            '*' => {
                self.advance();
                self.create_token(TokenType::Star, c.to_string())
            }
            '=' => {
                self.advance();
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.advance();
                    self.create_token(TokenType::EqualEqual, String::from("=="))
                } else {
                    self.create_token(TokenType::Equal, c.to_string())
                }
            }
            '!' => {
                self.advance();
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.advance();
                    self.create_token(TokenType::BangEqual, String::from("!="))
                } else {
                    self.create_token(TokenType::Bang, c.to_string())
                }
            }
            '<' => {
                self.advance();
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.advance();
                    self.create_token(TokenType::LessEqual, String::from("<="))
                } else {
                    self.create_token(TokenType::Less, c.to_string())
                }
            }
            '>' => {
                self.advance();
                if self.chars.peek().unwrap_or(&' ') == &'=' {
                    self.advance();
                    self.create_token(TokenType::GreaterEqual, String::from(">="))
                } else {
                    self.create_token(TokenType::Greater, c.to_string())
                }
            }
            _ => {
                // The below is a hack till I figure out how to just return the tokens
                self.advance();
                self.create_token(TokenType::Illegal, c.to_string())
            }
        }
    }

    pub fn advance(&mut self) {
        self.chars.next();
        self.current += 1;
        self.column += 1;
    }
}

/// Tests for single character tokens and operators
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_character_tokens_and_operators() {
        let input = String::from("=+(){}<=.<!>!=-==*");
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.scan_tokens();

        assert_eq!(
            tokens[0],
            Token::new(TokenType::Equal, String::from("="), 1, 1)
        );
        assert_eq!(
            tokens[1],
            Token::new(TokenType::Plus, String::from("+"), 1, 2)
        );
        assert_eq!(
            tokens[2],
            Token::new(TokenType::LeftParen, String::from("("), 1, 3)
        );
        assert_eq!(
            tokens[3],
            Token::new(TokenType::RightParen, String::from(")"), 1, 4)
        );
        assert_eq!(
            tokens[4],
            Token::new(TokenType::LeftBrace, String::from("{"), 1, 5)
        );
        assert_eq!(
            tokens[5],
            Token::new(TokenType::RightBrace, String::from("}"), 1, 6)
        );
        assert_eq!(
            tokens[6],
            Token::new(TokenType::LessEqual, String::from("<="), 1, 8)
        );
        assert_eq!(
            tokens[7],
            Token::new(TokenType::Dot, String::from("."), 1, 9)
        );
        assert_eq!(
            tokens[8],
            Token::new(TokenType::Less, String::from("<"), 1, 10)
        );
        assert_eq!(
            tokens[9],
            Token::new(TokenType::Bang, String::from("!"), 1, 11)
        );
        assert_eq!(
            tokens[10],
            Token::new(TokenType::Greater, String::from(">"), 1, 12)
        );
        assert_eq!(
            tokens[11],
            Token::new(TokenType::BangEqual, String::from("!="), 1, 14)
        );
        assert_eq!(
            tokens[12],
            Token::new(TokenType::Minus, String::from("-"), 1, 15)
        );
        assert_eq!(
            tokens[13],
            Token::new(TokenType::EqualEqual, String::from("=="), 1, 17)
        );
        assert_eq!(
            tokens[14],
            Token::new(TokenType::Star, String::from("*"), 1, 18)
        );
        assert_eq!(
            tokens[15],
            Token::new(TokenType::EOF, String::from(""), 1, 18)
        );
    }

    #[test]
    fn skip_whitespace() {
        let input = String::from("  \t\t+    ");
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.scan_tokens();

        assert_eq!(
            tokens[0],
            Token::new(TokenType::Plus, String::from("+"), 1, 5)
        );
        assert_eq!(
            tokens[1],
            Token::new(TokenType::EOF, String::from(""), 1, 9)
        );
    }
}
