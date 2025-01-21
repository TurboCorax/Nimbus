use crate::frontend::parser::token::TokenType::*;
use crate::frontend::parser::token::{Token, TokenType};
use crate::utils::error::{Error, ErrorHandler, ErrorType};

pub struct Lexer {
    content: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    tokens: Vec<Card>,
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

    pub(crate) fn scan_tokens(mut self) -> Result<Vec<Card>, ErrorHandler> {
        while !self.is_eof() {
            self.scan_token();

            if let Some(Token {
                            token_type: End,
                            ..
                        }) = self.tokens.last()
            {
                break;
            }
        }
        self.add_token(Eof, String::new());

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
            '-' => self.add_token(Minus, "-".to_string()),
            '+' => self.add_token(Add, "+".to_string()),
            '(' => self.add_token(LParen, "(".to_string()),
            ')' => self.add_token(RParen, ")".to_string()),
            '{' => self.add_token(LBrace, "{".to_string()),
            '}' => self.add_token(RBrace, "}".to_string()),
            '=' => self.add_token(Equal, "=".to_string()),
            '.' => self.command(),
            ',' => self.add_token(Comma, ",".to_string()),
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
        // self.tokens?.last().cloned()
        self.tokens.last()?.last()?.cloned()
    }

    fn peek_next(&self) -> char {
        self.content.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, content: String) {
        let token: Token = Token::new(token_type, self.line, self.column, content);
        if let Some(card) = self.tokens.last_mut() {
            // existing card must have at least one token
            match card.tokens.last() {
                Some(t)=>{
                    if t.line<token.line{
                        self.tokens.push(Card { tokens: vec![token] });
                    }
                }
                None=>{
                    card.tokens.push(token);
                }
            }
        } else {
            self.tokens.push(Card { tokens: vec![token] });
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
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
        let consume_simple_number = |lexer: &mut Lexer| {
            while let Some(c) = lexer.peek() {
                if !lexer.is_digit(c) {
                    break;
                }
                lexer.advance();
            }
        };
        self.start = self.current;

        consume_simple_number(self);

        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            self.advance();
            consume_simple_number(self);
        }

        self.add_token(
            Number,
            self.content[self.start - 1..self.current].to_string(),
        );

        // if next !=e/E, then it is a unit
        if let Some(c) = self.peek() {
            if self.is_alpha(c) && self.peek() == Some('e') && self.peek() == Some('E') {
                self.advance();
                self.add_token(E, "E".to_string());

                match self.peek() {
                    Some('-') => {
                        self.advance();
                        self.add_token(Minus, "-".to_string());
                    }
                    Some('+') => {
                        self.advance();
                        self.add_token(Add, "+".to_string());
                    }
                    _ => {}
                }
                // read the exponent
                consume_simple_number(self);

                self.add_token(
                    Number,
                    self.content[self.start - 1..self.current].to_string(),
                );
            }
        }
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
                                token_type: Number,
                                ..
                            }) = self.last_token()
                {
                    let last = self.last(2);
                    if last.is_some() && last.unwrap().is_digit(10) {
                        self.add_token(E, "E".to_string());
                        return;
                    }
                }
            }
            _ => {}
        }

        // Read the rest of the string
        while let Some(c) = self.peek() {
            if !self.is_alphanumeric(c) {
                break;
            }
            self.advance();
        }

        self.add_token(
            Identifier,
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
            ".end" => self.add_token(End, command),
            ".ends" => self.add_token(Ends, command),
            ".tran" => self.add_token(Tran, command),
            ".dc" => self.add_token(Dc, command),
            ".ac" => self.add_token(Ac, command),
            ".op" => self.add_token(Op, command),
            ".subckt" => self.add_token(Subckt, command),
            ".plot" => self.add_token(Plot, command),
            ".wave" => self.add_token(Wave, command),
            c => self.error_handler.add_error(Error::new(
                ErrorType::Lexical,
                format!("Unexpected command: {}", c),
                self.line,
                self.column,
            )),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub tokens: Vec<Token>,
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::utils::dbg;

    #[test]
    fn test_lexer_1() {
        let source = "* This is a comment\n.end\n";
        let lexer = Lexer::new(source.into());
        match lexer.scan_tokens() {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 1 + 1);
                assert_eq!(tokens[0][0].token_type, End);
            }
            Err(errors) => {
                errors.report_errors();
                panic!();
            }
        }
    }

    #[test]
    fn test_pure_comment() {
        let source = "* This is a comment\n";
        let lexer = Lexer::new(source.into());
        match lexer.scan_tokens() {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 1);
            }
            Err(errors) => {
                errors.report_errors();
                panic!();
            }
        }
    }

    #[test]
    fn test_lexer_2() {
        // R1 R1_1 R1_2 13.12e6m;This is a comment
        // V1 R1_1 GND 1
        // .wave V(XSC1_A) V(XSC1_B)
        // .end
        let source = "R1 R1_1 R1_2 13.12e6m;This is a comment\nV1 R1_1 GND 1 \n.wave V(XSC1_A) V(XSC1_B)\n.end\n";
        let lexer = Lexer::new(source.into());
        let result = lexer.scan_tokens();
        match result {
            Ok(tokens) => {
                dbg!(&tokens);
                assert_eq!(tokens.len(), 22);
            }
            Err(errors) => {
                errors.report_errors();
                panic!("Error");
            }
        }
    }

    #[test]
    fn test_lexer_3() {
        // ** Sheet_1 **
        // R1 R1_1 R1_2 1K
        // C1 C1_1 R1_2 1SADF; incorrect unit will be lexed as normal unit and ignored by the parser
        // L2 GND C1_1 1M
        // XSC1 C1_1 GND GND GND R1_1 GND XSC1_A XSC1_B OSCILLOSCOPE
        // V1 R1_1 GND 1
        // .wave V(XSC1_A) V(XSC1_B)
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
        let source = "-1.2E+3.4m";
        let lexer = Lexer::new(source.into());
        let result = lexer.scan_tokens();
        dbg(&result.unwrap());
    }
}
