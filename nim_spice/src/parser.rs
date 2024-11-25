use crate::command::Command;
use crate::error::{Error, ErrorHandler, ErrorType};
use crate::token::Token;
use crate::token::TokenType;

static UNITS: [&str; 9] = ["T", "G", "MEG", "K", "M", "U", "N", "P", "F"];

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    error_handler: ErrorHandler,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            tokens: vec![],
            current: 0,
            error_handler: ErrorHandler::new(),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            ..Self::default()
        }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        while !self.is_eof() {
            let token = self.advance();
            let pre = token.content.chars().nth(0);
            match pre {
                Some('.') => {
                    todo!()
                }
                Some(_) => {
                    self.error_handler.add_error(Error::new(
                        ErrorType::Syntax,
                        "Expected command".to_string(),
                        token.line,
                        token.column,
                    ));
                }
                None => {
                    self.error_handler.add_error(Error::new(
                        ErrorType::Syntax,
                        "Expected command".to_string(),
                        token.line,
                        token.column,
                    ));
                }
            }
        }
        todo!()
    }

    pub fn parse_num(&mut self) -> f64 {
        let mut token = self.advance();
        let mut n: f64 = match token.token_type {
            TokenType::Number => token.content.parse().unwrap(),
            t => {
                self.error_handler.add_error(Error::new(
                    ErrorType::Syntax,
                    format!("Expected number, found {:?}", t),
                    token.line,
                    token.column,
                ));
                0.0
            }
        };

        // If E notation is used, get one more number
        if let Token {
            token_type: TokenType::E,
            ..
        } = self.peek()
        {
            self.advance(); // consume E
            let mut num = self.advance();
            if TokenType::Number == num.token_type {
                let exp: f64 = num.content.parse().unwrap();
                n *= 10_f64.powf(exp);
            } else {
                self.error_handler.add_error(Error::new(
                    ErrorType::Syntax,
                    "Expected number after E".to_string(),
                    num.line,
                    num.column,
                ));
            }
        }

        // Look for unit
        if let Token {
            token_type: TokenType::Unit,
            ..
        } = self.peek()
        {
            let unit = self.advance();
            if UNITS.contains(&unit.content.as_str()) {
                let unit = unit.content;
                // TODO
                // If an available unit name happens as the prefix of an unknown unit,
                // it should be detected
                match unit.as_str() {
                    "T" => n *= 1e12,
                    "G" => n *= 1e9,
                    "MEG" => n *= 1e6,
                    "K" => n *= 1e3,
                    "M" => n *= 1e-3,
                    "U" => n *= 1e-6,
                    "N" => n *= 1e-9,
                    "P" => n *= 1e-12,
                    "F" => n *= 1e-15,
                    // Auto ignore unknown units
                    _ => {}
                }
            }
        }

        n
    }

    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn next(&self) -> Token {
        self.tokens[self.current + 1].clone()
    }

    fn matches(&mut self, token_type: TokenType) -> bool {
        if self.tokens[self.current].token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn is_eof(&self) -> bool {
        self.tokens[self.current].token_type == TokenType::Eof
    }
}
