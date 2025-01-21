use crate::circuit::Circuit;
use crate::frontend::parser::lexer::Card;
use crate::frontend::parser::netlist::Netlist;
use crate::frontend::parser::token::Token;
use crate::frontend::parser::token::TokenType;
use crate::frontend::parser::token::TokenType::Number;
use crate::frontend::parser::sym_table::SymTable;
use crate::utils::error::{Error, ErrorHandler, ErrorType};

static UNITS: [&str; 9] = ["T", "G", "MEG", "K", "M", "U", "N", "P", "F"];

struct Parser {
    tokens: Vec<Card>,
    last_line: usize, // Line number of the last advanced token
    current: usize,
    current_card: usize,
    error_handler: ErrorHandler,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            tokens: vec![],
            last_line: 0,
            current: 0,
            current_card: 0,
            error_handler: ErrorHandler::new(),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Card>) -> Self {
        Parser {
            tokens,
            ..Self::default()
        }
    }

    pub fn parse(&mut self) -> Netlist {
        let mut circuit = Circuit::new();
        let mut sym_table = SymTable::new();

        // TODO: find ground node and add it to the circuit

        while !self.is_eof() {
            let token = self.peek();
            let pre = token.content.chars().nth(0).unwrap();

            match pre {
                'R' => self.parse_resistor(&mut circuit, &mut sym_table),
                'C' => self.parse_capacitor(&mut circuit, &mut sym_table),
                'L' => self.parse_inductor(&mut circuit, &mut sym_table),
                _ => {
                    // Unknown
                    self.error_handler.add_error(Error::new(
                        ErrorType::Syntax,
                        format!("Unknown leading character: {}", pre),
                        token.line,
                        token.column,
                    ));
                }
            }

            self.current_card += 1;
        }
        todo!()
    }

    pub fn parse_num(&mut self) -> f64 {
        let mut token = self.advance();
        let mut n: f64 = match token.token_type {
            Number => token.content.parse().unwrap(),
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
            if Number == num.token_type {
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

    fn advance(&mut self) ->Token {
        let token= self.tokens[self.current_card].tokens[self.current].clone();
        self.current += 1;
        if self.is_at_end() {
            self.current_card += 1;
            self.current = 0;
        }
        token
    }

    fn is_at_end(&self) -> bool {
        // eof
        self.tokens[self.current_card].tokens.len() == self.current
    }

    fn peek(&self) -> Token {
        self.tokens[self.current_card].tokens[self.current].clone()
    }

    fn next(&self) -> Token {
        self.tokens[self.current_card].tokens[self.current + 1].clone()
    }

    fn matches(&mut self, token_type: Vec<TokenType>) -> bool {
        if token_type.contains(&self.tokens[self.current_card].tokens[self.current].token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_eof(&self) -> bool {
        self.tokens[self.current_card].tokens[self.current].token_type == TokenType::Eof
    }

    fn parse_resistor(&mut self, circuit: &mut Circuit, sym_table: &mut SymTable) {
        let name = self.advance();
        
        // two nodes
        let node1 = self.advance();
        let node2 = self.advance();
        
        todo!()
    }

    fn parse_capacitor(&mut self, circuit: &mut Circuit, sym_table: &mut SymTable) {
        todo!()
    }

    fn parse_inductor(&mut self, circuit: &mut Circuit, sym_table: &mut SymTable) {
        todo!()
    }
}

