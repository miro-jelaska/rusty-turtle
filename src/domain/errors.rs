use std::fmt::Display;

#[derive(Debug)]
pub enum InterpreterError {
    Tokenize {
        line: usize,
        message: String,
    },
    Parser {
        line: usize,
        message: String
    },
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::Tokenize { line, message } => write!(f, "Tokenizer Error at line {}: {}", line, message),
            InterpreterError::Parser { line, message } => write!(f, "Parser Error at line{}: {}", line, message),
        }
    }
}
