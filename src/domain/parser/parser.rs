use std::iter::Peekable;

use log::info;

use crate::domain::errors::InterpreterError;
use crate::domain::parser::expressions::{Expr, Literal};
use crate::domain::parser::statements::{MoveBackwardStmt, MoveForwardStmt, RepeatStmt, RotateLeftStmt, RotateRightStmt, SetColorStmt, Stmt};
use crate::domain::parser::statements::Stmt::Block;
use crate::domain::tokenizer::token::{Token, TokenType};
use crate::domain::tokenizer::token::TokenType::{LeftBracketToken, RightBracketToken};
use crate::domain::tokenizer::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new_from_str(source: &'a str) -> Self {
        Parser {
            tokens: Tokenizer::new(source).peekable(),
        }
    }

    fn make_error<S>(&mut self, message: S) -> InterpreterError
        where
            S: Into<String>,
    {
        InterpreterError::Parser {
            line: self.tokens.peek().map(Token::line).unwrap_or(0),
            message: message.into(),
        }
    }

    fn peek_is(&mut self, token_type: TokenType) -> bool {
        match self.tokens.peek() {
            Some(x) => x.is_type(token_type),
            _ => false
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, InterpreterError> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();
        while !self.has_reached_end() {
            match self.parse_declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    errors.push(err);
                    // We don't synchronise after an error.
                    // Instead, we terminate parsing immediately.
                    break;
                }
            }
        }
        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors.pop().unwrap())
        }
    }

    fn consume_if(&mut self, token_type: TokenType) -> Option<Token> {
        if self.peek_is(token_type) {
            return self.tokens.next();
        }
        None
    }

    fn consume_expecting(
        &mut self,
        token_type: TokenType,
        error_message: &'static str,
    ) -> Result<Token, InterpreterError> {
        if let Some(token) = self.consume_if(token_type) {
            Ok(token)
        } else {
            Err(
                InterpreterError::Parser {
                    line: self.tokens.peek().map(Token::line).unwrap_or(0),
                    message: error_message.into(),
                }
            )
        }
    }

    fn consume(&mut self) -> Option<Token> {
        // Consumes next token regardless of its type.
        self.tokens.next()
    }

    pub fn consume_block(&mut self) -> Result<Vec<Token>, InterpreterError> {
        // Expects to start with [ and end with ]
        let _ = self.consume_expecting(LeftBracketToken, "Expected block. Block has to start with opening bracket `[`. Opening bracket is missing.");
        let mut open_brackets_count = 1;
        let mut all_block_tokens: Vec<Token> = Vec::new();
        while !self.has_reached_end() {
            let peek =
                self.tokens.peek()
                .expect("Must exist because tokenizer did not reach the end yet.");
            match peek {
                x if x.is_type(LeftBracketToken) => {
                    open_brackets_count += 1;
                    all_block_tokens.push(self.consume().unwrap());
                },
                x if x.is_type(RightBracketToken) => {
                    if open_brackets_count < 1 {
                        return Err(InterpreterError::Parser {
                            // FIXME: The line number has to come from the token.
                            line: 0,
                            message: "Invalid number of closing brackets inside a block. There are more closing brackets than expected.".into()
                        })
                    }
                    if open_brackets_count == 1 {
                        let _ = self.consume_expecting(RightBracketToken, "Expected block. Block has to end with closing bracket `]`. Closing bracket is missing.");
                        return Ok(all_block_tokens);
                    }
                    open_brackets_count -= 1;
                    all_block_tokens.push(self.consume().unwrap());
                },
                _ => all_block_tokens.push(self.consume().unwrap())
            }
        }
        Err(InterpreterError::Parser {
            // FIXME: The line number has to come from the last token.
            line: 0,
            message: "Invalid number of closing brackets inside a block. There are more closing brackets than expected.".into()
        })
    }

    fn has_reached_end(&mut self) -> bool {
        self.tokens.peek().is_none()
    }

    fn parse_declaration(&mut self) -> Result<Stmt, InterpreterError> {
        // Branching would occur here if language had support for functions, classes and variables.
        // For now, Rusty Turtle only supports statements.
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Result<Stmt, InterpreterError> {
        if self.consume_if(TokenType::RepeatLoopToken).is_some() {
            self.parse_repeat_loop()
        } else if self.consume_if(TokenType::SetColorCommandToken).is_some() {
            self.parse_set_color_command_stmt()
        } else if self.consume_if(TokenType::ForwardCommandToken).is_some() {
            self.parse_forward_command_stmt()
        } else if self.consume_if(TokenType::BackwardCommandToken).is_some() {
            self.parse_back_command_stmt()
        } else if self.consume_if(TokenType::TurnRightCommandToken).is_some() {
            self.parse_right_command_stmt()
        } else if self.consume_if(TokenType::TurnLeftCommandToken).is_some() {
            self.parse_left_command_stmt()
        }else {
            self.parse_expression_stmt()
        }
    }

    fn parse_set_color_command_stmt(&mut self)-> Result<Stmt, InterpreterError> {
        let color = self.consume_expecting(TokenType::ColorToken, "Expecting HEX color after COLOR command.")?;
        match color {
            Token::ColorToken(_, _) => Ok(Stmt::SetColor(SetColorStmt {color: color.lexeme()})),
            _ => Err(self.make_error("Expected number."))
        }
    }

    fn parse_forward_command_stmt(&mut self)-> Result<Stmt, InterpreterError> {
        let number = self.consume_expecting(TokenType::NumberToken, "Expecting number after forward command.")?;
        match number {
            Token::NumberToken(_, value) => Ok(Stmt::MoveForward(MoveForwardStmt {distance: value})),
            _ => Err(self.make_error("Expected number."))
        }
    }

    fn parse_back_command_stmt(&mut self)-> Result<Stmt, InterpreterError> {
        let number = self.consume_expecting(TokenType::NumberToken, "Expecting number after back command.")?;
        match number {
            Token::NumberToken(_, value) => Ok(Stmt::MoveBack(MoveBackwardStmt {distance: value})),
            _ => Err(self.make_error("Expected number."))
        }
    }

    fn parse_right_command_stmt(&mut self)-> Result<Stmt, InterpreterError> {
        let number = self.consume_expecting(TokenType::NumberToken, "Expecting number after turn right command.")?;
        match number {
            Token::NumberToken(_, value) => Ok(Stmt::RotateRight(RotateRightStmt {angular_distance: value})),
            _ => Err(self.make_error("Expected number."))
        }
    }

    fn parse_left_command_stmt(&mut self)-> Result<Stmt, InterpreterError> {
        let number = self.consume_expecting(TokenType::NumberToken, "Expecting number after turn left command.")?;
        match number {
            Token::NumberToken(_, value) => Ok(Stmt::RotateLeft(RotateLeftStmt {angular_distance: value})),
            _ => Err(self.make_error("Expected number."))
        }
    }

    fn parse_expression_stmt(&mut self) -> Result<Stmt, InterpreterError> {
        // Currently Rust Turtle only supports Number literals.
        Ok(Stmt::Expression(self.parse_primary()?))
    }
    fn parse_primary(&mut self) -> Result<Expr, InterpreterError> {
        if let Some(token) = self.consume_if(TokenType::NumberToken) {
            return match token {
                Token::NumberToken(_, value) => Ok(Expr::Literal(Literal::Number(value))),
                _ => panic!("Expected Number"),
            }
        }
        info!("token {}", self.tokens.peek().unwrap().lexeme());
        if let Some(token) = self.consume_if(TokenType::ColorToken){
            return match token {
                Token::ColorToken(_, value) => Ok(Expr::Literal(Literal::Color(value))),
                _ => panic!("Expected Color"),
            }
        }
        Err(self.make_error("Expected an expression while parsing primary."))
    }
    fn parse_repeat_loop(&mut self) -> Result<Stmt, InterpreterError> {
        let number_of_repeats_token =
            self.consume_expecting(
                TokenType::NumberToken,
                "Repeat statement must define a number of repeats. Parser didn't find number."
            )?;
        let number_of_repeats = match number_of_repeats_token {
            Token::NumberToken(_, value) => value.round() as usize,
            _ => panic!("TODO")
        };
        let block_tokens = self.consume_block()?;
        // This is not an ideal approach, but it should work fine for Rusty Turtle purpose.
        // We turn tokens back into a string so we can feed a string to a new parsers that will parse only that block of code.
        // Some inaccuracies are possible, like with turning NumberToken back to its string representation.
        let block_code: String =
            block_tokens
                .iter()
                .map(|x| x.lexeme().to_string())
                .collect::<Vec<String>>()
                .join(" ");
        let mut block_parser = Parser::new_from_str(block_code.as_ref());
        let block_statements = block_parser.parse()?;
        Ok(Stmt::Repeat(
            RepeatStmt {
                count: number_of_repeats,
                body: Box::new(Block(block_statements))
        }))
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::parser::parser::Parser;

    #[test]
    fn use_tokenizers_next() {
        // let result = Parser::new_from_str("REPEAT 30 [ FD 40 REPEAT 5 [ FD 60 FD 40 ] ]").parse();
        let result = Parser::new_from_str("REPEAT 30 [ FD 40 FD 50 ] BK 40").parse();
        println!("done")
    }
}