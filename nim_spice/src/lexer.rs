use crate::error::{Error, ErrorHandler, ErrorType};
use crate::token::{Token, TokenType};

pub struct Lexer {
    content: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    error_handler: ErrorHandler,
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

    pub(crate) fn scan_tokens(mut self) -> Result<Vec<Token>, ErrorHandler> {
        while !self.is_eof() {
            self.scan_token();
            if self.tokens.last().unwrap().token_type == TokenType::End {
                break;
            }
        }
        self.add_token(TokenType::Eof, String::new());

        if !self.error_handler.has_errors() {
            Ok(self.tokens)
        } else {
            Err(self.error_handler)
        }
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '*' | ';' => {
                while !self.is_eof() && self.peek() != Some('\n') {
                    self.advance();
                }
            }
            '-' => self.add_token(TokenType::Minus, "-".to_string()),
            '+' => self.add_token(TokenType::Add, "+".to_string()),
            '(' => self.add_token(TokenType::LParen, "(".to_string()),
            ')' => self.add_token(TokenType::RParen, ")".to_string()),
            '{' => self.add_token(TokenType::LBrace, "{".to_string()),
            '}' => self.add_token(TokenType::RBrace, "}".to_string()),
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

    fn peek(&self) -> Option<char> {
        self.content.chars().nth(self.current)
    }

    fn last(&self, step: usize) -> Option<char> {
        self.content.chars().nth(self.current - step)
    }

    fn last_token(&self) -> Option<Token> {
        self.tokens.last().cloned()
    }

    fn peek_next(&self) -> char {
        self.content.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, content: String) {
        let token: Token = Token::new(token_type, self.line, self.column, content);
        dbg!(&token);
        self.tokens.push(token);
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

    // Read an integer or a float number
    // TODO: Support numbers like 6K34
    fn number(&mut self) {
        self.start = self.current;

        while let Some(c) = self.peek() {
            if !self.is_digit(c) {
                break;
            }
            self.advance();
        }

        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            self.advance();
            while let Some(c) = self.peek() {
                if !self.is_digit(c) {
                    break;
                }
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            self.content[self.start - 1..self.current].to_string(),
        );

        // if next !=e/E, then it is a unit
        if let Some(c) = self.peek() {
            if self.is_alpha(c) && !(self.peek() != Some('e') && self.peek() != Some('E')) {
                return;
            }
            self.unit();
        }
    }

    fn unit(&mut self) {
        self.start = self.current;

        while let Some(c) = self.peek() {
            if !self.is_alphanumeric(c) {
                break;
            }
            self.advance();
        }

        self.add_token(
            TokenType::Unit,
            self.content[self.start..self.current].to_string(),
        );
    }

    fn identifier(&mut self) {
        self.start = self.current - 1;

        // Read the prefix alpha string only
        while let Some(c) = self.peek() {
            if !self.is_alpha(c) {
                break;
            }
            self.advance();
        }

        match self.content[self.start..self.current].into() {
            "E" | "e" => {
                // if E/e is just after a number, then it is an exponent
                if let Some(Token {
                                token_type: TokenType::Number,
                                ..
                            }) = self.last_token()
                {
                    let last = self.last(2);
                    if last.is_some() && last.unwrap().is_digit(10) {
                        self.add_token(TokenType::E, "E".to_string());
                        return;
                    }
                }
            }
            _ => {}
        }

        while let Some(c) = self.peek() {
            if !self.is_alphanumeric(c) {
                break;
            }
            self.advance();
        }

        let token_type = match self.last(1) {
            Some(c) if self.is_digit(c) => TokenType::Unit,
            _ => TokenType::Identifier,
        };
        self.add_token(
            token_type,
            self.content[self.start..self.current].to_string(),
        );
    }

    fn command(&mut self) {
        self.start = self.current - 1;

        while let Some(c) = self.peek() {
            if !self.is_alpha(c) {
                break;
            }
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

#[cfg(test)]
mod test {
    use super::*;
    use bevy::utils::dbg;

    #[test]
    fn test_lexer_1() {
        let source = "* This is a comment\n.end\n";
        let mut lexer = Lexer::new(source.into());
        match lexer.scan_tokens() {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 1 + 1);
                assert_eq!(tokens[0].token_type, TokenType::End);
            }
            Err(errors) => {
                errors.report_errors();
                assert!(false);
            }
        }
    }

    #[test]
    fn test_lexer_2() {
        ///R1 R1_1 R1_2 13.12e6m;This is a comment
        // V1 R1_1 GND 1
        // .wave V(XSC1_A) V(XSC1_B)
        // .end
        let source = "R1 R1_1 R1_2 13.12e6m;This is a comment\nV1 R1_1 GND 1 \n.wave V(XSC1_A) V(XSC1_B)\n.end\n";
        let mut lexer = Lexer::new(source.into());
        let result = lexer.scan_tokens();
        match result {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 19 + 1);
            }
            Err(errors) => {
                errors.report_errors();
                assert!(false);
            }
        }
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
        let result = lexer.scan_tokens();
        match result {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 75);
            }
            Err(errors) => {
                errors.report_errors();
                assert!(false);
            }
        }
    }

    #[test]
    fn test_number() {
        let source = "1.2E3.4m";
        let mut lexer = Lexer::new(source.into());
        let result = lexer.scan_tokens();
        dbg(&result.unwrap());
    }
}
