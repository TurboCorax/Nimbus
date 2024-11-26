use crate::frontend::parser::netlist::Netlist;
use crate::frontend::parser::token::Token;
use crate::frontend::parser::token::TokenType;
use crate::utils::error::{Error, ErrorHandler, ErrorType};

static UNITS: [&str; 9] = ["T", "G", "MEG", "K", "M", "U", "N", "P", "F"];

struct Parser {
    tokens: Vec<Token>,
    last_line: usize, // Line number of the last advanced token
    current: usize,
    error_handler: ErrorHandler,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            tokens: vec![],
            last_line: 0,
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

    pub fn parse(&mut self) -> Netlist {
        let mut net = Netlist::new();

        while !self.is_eof() {
            let token = self.advance();
            let pre = token.content.chars().nth(0);
            /*
            Leading Character - Type of line

            * Comment  // No need to parse
            A: Special function device
            B: Arbitrary behavioral source
            C: Capacitor
            D: Diode
            E: Voltage dependent voltage source
            F: Current dependent current source
            G: Voltage dependent current source
            H: Current dependent voltage source
            I: Independent current source
            J: JFET transistor
            K: Mutual inductance
            L: Inductor
            M: MOSFET transistor
            O: Lossy transmission line
            Q: Bipolar transistor
            R: Resistor
            S: Voltage controlled switch
            T: Lossless transmission line
            U: Uniform RC-line
            V: Independent voltage source
            W: Current controlled switch
            X: Subcircuit Invocation
            Z: MESFET transistor
            .: A simulation directive, For example: .options reltol=1e-4
            +: A continuation of the previous line. The "+" is removed and the remainder of the line is considered part of the prior line.
             */
            if let None = pre {
                todo!();
                return net;
            }

            match pre.unwrap() {
                'A' => {
                    // Special function device
                    self.special_function_device();
                }
                'B' => {
                    // Arbitrary behavioral source
                    self.arbitrary_behavioral_source();
                }
                'C' => {
                    // Capacitor
                    self.capacitor();
                }
                'D' => {
                    // Diode
                    self.diode();
                }
                'E' => {
                    // Voltage dependent voltage source
                    self.voltage_dependent_voltage_source();
                }
                'F' => {
                    // Current dependent current source
                    self.current_dependent_current_source();
                }
                'G' => {
                    // Voltage dependent current source
                    self.voltage_dependent_current_source();
                }
                'H' => {
                    // Current dependent voltage source
                    self.current_dependent_voltage_source();
                }
                'I' => {
                    // Independent current source
                    self.independent_current_source();
                }
                'J' => {
                    // JFET transistor
                    self.jfet_transistor();
                }
                'K' => {
                    // Mutual inductance
                    self.mutual_inductance();
                }
                'L' => {
                    // Inductor
                    self.inductor();
                }
                'M' => {
                    // MOSFET transistor
                    self.mosfet_transistor();
                }
                'O' => {
                    // Lossy transmission line
                    self.lossy_transmission_line();
                }
                'Q' => {
                    // Bipolar transistor
                    self.bipolar_transistor();
                }
                'R' => {
                    // Resistor
                    self.resistor();
                }
                'S' => {
                    // Voltage controlled switch
                    self.voltage_controlled_switch();
                }
                'T' => {
                    // Lossless transmission line
                    self.lossless_transmission_line();
                }
                'U' => {
                    // Uniform RC-line
                    self.uniform_rc_line();
                }
                'V' => {
                    // Independent voltage source
                    self.independent_voltage_source();
                }
                'W' => {
                    // Current controlled switch
                    self.current_controlled_switch();
                }
                'X' => {
                    // Subcircuit Invocation
                    self.subcircuit_invocation();
                }
                'Z' => {
                    // MESFET transistor
                    self.mesfet_transistor();
                }
                '.' => {
                    // Simulation directive
                    self.simulation_directive();
                }
                '+' => {
                    // Continuation of the previous line
                    self.continuation_of_the_previous_line();
                }
                ' ' => {
                    // Empty line
                    continue;
                }
                _ => {
                    // Unknown
                    self.error_handler.add_error(Error::new(
                        ErrorType::Syntax,
                        format!("Unknown leading character: {}", pre.unwrap()),
                        token.line,
                        token.column,
                    ));
                }
            }
        }
        net
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

    fn special_function_device(&mut self) {
        todo!()
    }

    fn arbitrary_behavioral_source(&mut self) {
        todo!()
    }

    fn capacitor(&mut self) {
        todo!()
    }

    fn diode(&mut self) {
        todo!()
    }

    fn voltage_dependent_voltage_source(&mut self) {
        todo!()
    }

    fn current_dependent_current_source(&mut self) {
        todo!()
    }

    fn voltage_dependent_current_source(&mut self) {
        todo!()
    }

    fn current_dependent_voltage_source(&mut self) {
        todo!()
    }

    fn independent_current_source(&mut self) {
        todo!()
    }

    fn jfet_transistor(&mut self) {
        todo!()
    }

    fn mutual_inductance(&mut self) {
        todo!()
    }

    fn inductor(&mut self) {
        todo!()
    }

    fn mosfet_transistor(&mut self) {
        todo!()
    }

    fn lossy_transmission_line(&mut self) {
        todo!()
    }

    fn bipolar_transistor(&mut self) {
        todo!()
    }

    fn resistor(&mut self) {
        todo!()
    }

    fn voltage_controlled_switch(&mut self) {
        todo!()
    }

    fn lossless_transmission_line(&mut self) {
        todo!()
    }

    fn uniform_rc_line(&mut self) {
        todo!()
    }

    fn independent_voltage_source(&mut self) {
        todo!()
    }

    fn current_controlled_switch(&mut self) {
        todo!()
    }

    fn subcircuit_invocation(&mut self) {
        todo!()
    }

    fn mesfet_transistor(&mut self) {
        todo!()
    }

    fn simulation_directive(&mut self) {
        todo!()
    }

    fn continuation_of_the_previous_line(&mut self) {
        todo!()
    }
}
