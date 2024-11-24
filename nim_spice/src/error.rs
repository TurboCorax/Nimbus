use std::fmt::Display;

// src/error.rs
#[derive(Debug)]
pub enum ErrorType {
    Lexical,
    Syntax,
    Semantic,
    Runtime,
    IOError,
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String, line: usize, column: usize) -> Self {
        Error {
            error_type,
            message,
            line,
            column,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "[{:?} Error] Line: {}, Column: {}: {}",
            self.error_type, self.line, self.column, self.message
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?} Error] Line: {}, Column: {}: {}",
            self.error_type, self.line, self.column, self.message
        )
    }
}

pub struct ErrorHandler {
    errors: Vec<Error>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_format() {
        let error = Error::new(ErrorType::Lexical, "Unexpected character".to_string(), 1, 1);
        assert_eq!(
            error.format(),
            "[Lexical Error] Line: 1, Column: 1: Unexpected character"
        );
    }

    #[test]
    fn test_error_handler() {
        let mut error_handler = ErrorHandler::new();
        let error = Error::new(ErrorType::Lexical, "Unexpected character".to_string(), 1, 1);
        error_handler.add_error(error);
        assert!(error_handler.has_errors());
    }
}