use crate::error::{Error, ErrorHandler, ErrorType};
use crate::token_type::TokenType;

static UNITS: [&str; 9] = ["T", "G", "MEG", "K", "M", "U", "N", "P", "F"];

struct Lexer {
    content: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    error_handler: ErrorHandler,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    line: usize,
    content: String,
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer {
            content: String::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            error_handler: ErrorHandler::new(),
        }
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            content: source,
            ..Default::default()
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.content.chars().nth(self.current - 1).unwrap()
    }

    fn scan_tokens(&mut self) {
        while !self.is_eof() {
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '*' | ';' => {
                while !self.is_eof() && self.peek() != '\n' {
                    self.advance();
                }
                // self.advance();
            }
            '(' => self.add_token(TokenType::LParen, "(".to_string()),
            ')' => self.add_token(TokenType::RParen, ")".to_string()),
            '=' => self.add_token(TokenType::Equal, "=".to_string()),
            '.' => self.command(),
            ',' => self.add_token(TokenType::Comma, ",".to_string()),
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            c if self.is_alpha(c) => self.identifier(),
            c if self.is_digit(c) => self.number(),
            c => self.error_handler.add_error(Error::new(
                ErrorType::Lexical,
                format!("Unexpected character: {}", c),
                self.line,
                self.column,
            )),
        }
    }

    fn is_eof(&self) -> bool {
        self.current >= self.content.len()
    }

    fn peek(&self) -> char {
        self.content.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        self.content.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, content: String) {
        self.tokens.push(Token {
            token_type,
            line: self.line,
            content,
        });
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_uppercase() || c.is_lowercase() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_comment(&self, c: char) -> bool {
        c == '*'
    }

    fn number(&mut self) {
        self.start = self.current;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        if self.peek() == 'e' || self.peek() == 'E' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(
            TokenType::Number,
            self.content[self.start - 1..self.current].to_string(),
        );
        if self.is_alpha(self.peek()) {
            self.unit();
        }
    }

    fn unit(&mut self) {
        self.start = self.current;

        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        self.add_token(
            TokenType::Unit,
            self.content[self.start..self.current].to_string(),
        );
    }

    fn identifier(&mut self) {
        self.start = self.current - 1;
        while self.is_alphanumeric(self.peek()) {
            // while !self.is_whitespace(self.peek()) && !self.is_line_end(self.peek()) {
            self.advance();
        }
        self.add_token(
            TokenType::Identifier,
            self.content[self.start..self.current].to_string(),
        );
    }

    fn command(&mut self) {
        self.start = self.current - 1;

        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let command = self.content[self.start..self.current].to_lowercase();
        match command.as_str() {
            ".end" => self.add_token(TokenType::End, command),
            ".ends" => self.add_token(TokenType::Ends, command),
            ".tran" => self.add_token(TokenType::Tran, command),
            ".dc" => self.add_token(TokenType::Dc, command),
            ".ac" => self.add_token(TokenType::Ac, command),
            ".op" => self.add_token(TokenType::Op, command),
            ".subckt" => self.add_token(TokenType::Subckt, command),
            ".plot" => self.add_token(TokenType::Plot, command),
            ".wave" => self.add_token(TokenType::Wave, command),
            c => self.error_handler.add_error(Error::new(
                ErrorType::Lexical,
                format!("Unexpected command: {}", c),
                self.line,
                self.column,
            )),
        }
    }
}

// TODO
struct Parser {}

impl Default for Parser {
    fn default() -> Self {
        Parser {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let source = "* This is a comment\n.end\n";
        let mut lexer = Lexer::new(source.into());
        lexer.scan_tokens();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
        assert_eq!(lexer.tokens[0].token_type, TokenType::End);
    }

    #[test]
    fn test_lexer_2() {
        ///R1 R1_1 R1_2 13.12e6m;This is a comment
        // V1 R1_1 GND 1
        // .wave V(XSC1_A) V(XSC1_B)
        // .end
        let source = "R1 R1_1 R1_2 13.12e6m;This is a comment\nV1 R1_1 GND 1 \n.wave V(XSC1_A) V(XSC1_B)\n.end\n";
        let mut lexer = Lexer::new(source.into());
        lexer.scan_tokens();
        lexer.error_handler.report_errors();
        assert_eq!(lexer.tokens.len(), 19);
        assert!(!lexer.error_handler.has_errors());
    }

    #[test]
    fn test_lexer_3() {
        ///** Sheet_1 **
        // R1 R1_1 R1_2 1K
        // C1 C1_1 R1_2 1SADF; incorrect unit will be lexed as normal unit and ignored by the parser
        // L2 GND C1_1 1M
        // XSC1 C1_1 GND GND GND R1_1 GND XSC1_A XSC1_B OSCILLOSCOPE
        // V1 R1_1 GND 1
        // .Save V(XSC1_A) V(XSC1_B)
        //
        // .SUBCKT  OSCILLOSCOPE 1  2  3 4 5 6 7 8
        // B1 7 GND V=V(1,2)
        // B2 8 GND V=V(3,4)
        // .ENDS
        //
        // .tran 10m
        let source = "** Sheet_1 ** \n R1 R1_1 R1_2 1K\n C1 C1_1 R1_2 1SADF\n L2 GND C1_1 1M\n XSC1 C1_1 GND GND GND R1_1 GND XSC1_A XSC1_B OSCILLOSCOPE \n V1 R1_1 GND 1\n .wave V(XSC1_A) V(XSC1_B) \n .SUBCKT  OSCILLOSCOPE 1  2  3 4 5 6 7 8\n B1 7 GND V=V(1,2)\n B2 8 GND V=V(3,4)\n .ENDS\n\n.tran 10m\n ";
        let mut lexer = Lexer::new(source.into());
        lexer.scan_tokens();
        lexer.error_handler.report_errors();
        dbg!(lexer.tokens);
        lexer.error_handler.report_errors();
        assert!(!lexer.error_handler.has_errors());
    }
}
