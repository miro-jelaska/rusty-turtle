use std::iter::Peekable;
use std::str::Chars;
use crate::domain::errors::InterpreterError;
use crate::domain::tokenizer::token::Token;
use phf::phf_map;

static KEYWORD_TO_TOKEN: phf::Map<&'static str, fn(usize) -> Token> = phf_map! {
    "repeat" => |line| Token::RepeatLoopToken(line),
    "color" => |line| Token::SetColorCommandToken(line),
    "forward" => |line| Token::ForwardCommandToken(line),
    "fd" => |line| Token::ForwardCommandToken(line),
    "back" => |line| Token::BackwardCommandToken(line),
    "bk" => |line| Token::BackwardCommandToken(line),
    "left" => |line| Token::TurnLeftCommandToken(line),
    "lt" => |line| Token::TurnLeftCommandToken(line),
    "right" => |line| Token::TurnRightCommandToken(line),
    "rt" => |line| Token::TurnRightCommandToken(line),
};

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    errors: Vec<InterpreterError>
}

impl<'a> Tokenizer<'a> {
    /*
    Implements public fn.
    */
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source: source.chars().into_iter().peekable(),
            line: 1,
            errors: vec!(),
        }
    }

    #[cfg(test)]
    pub fn read_tokens(&mut self) -> Vec<Token> {
        self.into_iter().collect()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let character = self.source.next()?;
        let token = match character {
            ' ' | '\t' | '\r' => return self.next(),
            '\n' => {
                self.line += 1;
                return self.next();
            }
            '[' => Token::LeftBracketToken(self.line),
            ']' => Token::RightBracketToken(self.line),
            current if current == '#' => self.consume_color(current),
            current if current.is_ascii_digit() => self.consume_number(current),
            current if is_alpha(&current) => self.consume_identifier(current),
            current => {
                self.errors.push(InterpreterError::Tokenize {
                    line: self.line,
                    message: format!("Unexpected character: {}", current),
                });
                Token::IllegalToken(self.line, current.to_string())
            }
        };
        Some(token)
    }
}

fn is_alpha(character: &char) -> bool {
    character.is_ascii_alphabetic() || *character == '_'
}

impl<'a> Tokenizer<'a> {
    /*
    Implements mechanics for scanning individual characters in the source code sequence.
    */

    fn peek_satisfies(&mut self, predicate: fn(&char) -> bool) -> bool {
        let peeked = self.source.peek();
        !peeked.is_none() && predicate(peeked.unwrap())
    }

    fn peek_of_peek_satisfies(&mut self, predicate: fn(&char) -> bool) -> bool {
        // We make alternative peekable so we don't mutate our primary from `self.source`.
        let mut peekable = self.source.clone().peekable();
        match peekable.next() {
            Some(_) => !peekable.peek().is_none() && predicate(peekable.peek().unwrap()),
            None => false
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.source.next().take()
    }

    fn consume_number(&mut self, first_character: char) -> Token {
        let mut number_chars = vec![first_character];
        while self.peek_satisfies(|x| x.is_numeric()) {
            number_chars.push(self.advance().unwrap());
        }
        if self.peek_satisfies(|x| *x == '.') && self.peek_of_peek_satisfies(|x| x.is_numeric()) {
            // Assumption: If we encounter a dot, it must be part of a number.
            // Alternatively, we can implement mechanism that can peek two characters in advance
            // so we don't consume dot unless we are sure a number comes after the dot.
            // We have already advanced over the dot so we want to add it.
            number_chars.push(self.advance().expect("Dot (`.`) character must exist."));
            while self.peek_satisfies(|x| x.is_numeric()) {
                number_chars.push(self.advance().unwrap());
            }
        }
        let number_as_str: String = number_chars.iter().collect();
        match number_as_str.parse::<f64>() {
            Ok(float) => Token::NumberToken(self.line, float),
            Err(_) => {
                self.errors.push(InterpreterError::Tokenize {
                    line: self.line,
                    message: format!("Expected number but could not parse it."),
                });
                Token::IllegalToken(self.line, "Expected number but value could not be parsed as a number.".into())
            },
        }
    }

    fn consume_identifier(&mut self, first: char) -> Token {
        let mut identifier_chars = vec![first];
        while self.peek_satisfies(|x| x.is_alphanumeric()) {
            identifier_chars.push(self.advance().unwrap());
        }
        let identifier_as_str: String = identifier_chars.iter().collect();
        match KEYWORD_TO_TOKEN.get(&identifier_as_str.to_ascii_lowercase()) {
            Some(keyword_token) => keyword_token(self.line),
            None => Token::IllegalToken(
                self.line,
                format!("The token is expected to be identifier. Identifier must be a keyword (functions, classes, and variables are not supported). Token `{}` does not match any keyword.", &identifier_as_str)
            ),
        }
    }

    pub fn consume_color(&mut self, first_character: char) -> Token {
        let mut parts = vec![first_character];
        for _ in 0..=5 {
            if self.peek_satisfies(|x| x.is_ascii_hexdigit() ){
                parts.push(self.advance().unwrap())
            } else {
                // TODO
            }
        }
        Token::ColorToken(
            self.line,
            parts.iter().collect()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::tokenizer::token::Token;
    use crate::domain::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn use_tokenizers_next() {
        let mut tokenizer = Tokenizer::new("left 10".into());
        let first_token = tokenizer.next();
        assert_eq!(first_token, Some(Token::TurnLeftCommandToken(1)));
        let second_token = tokenizer.next();
        assert_eq!(second_token, Some(Token::NumberToken(1, 10.0_f64)));
    }

    #[test]
    fn consuming_number() {
        let mut tokenizer = Tokenizer::new("0 10 55.5".into());
        let r = tokenizer.read_tokens();
        assert_eq!(r[0].clone(), Token::NumberToken(1, 0.0_f64));
        assert_eq!(r[1].clone(), Token::NumberToken(1, 10.0_f64));
        assert_eq!(r[2].clone(), Token::NumberToken(1, 55.5_f64));
    }

    #[test]
    fn consuming_command() {
        let mut tokenizer = Tokenizer::new(" rt 10 ".into());
        let r = tokenizer.read_tokens();
        assert_eq!(r[0].clone(), Token::TurnRightCommandToken(1));
        assert_eq!(r[1].clone(), Token::NumberToken(1, 10.0_f64));
    }

    #[test]
    fn commands_are_case_insensitive() {
        let mut tokenizer = Tokenizer::new(" RT left FoRwArD".into());
        let r = tokenizer.read_tokens();
        assert_eq!(r[0].clone(), Token::TurnRightCommandToken(1));
        assert_eq!(r[1].clone(), Token::TurnLeftCommandToken(1));
        assert_eq!(r[2].clone(), Token::ForwardCommandToken(1));
    }

    #[test]
    fn consuming_color() {
        let mut tokenizer = Tokenizer::new("COLOR #004355 RT 50".into());
        let r = tokenizer.read_tokens();
        assert_eq!(r[1].clone(), Token::ColorToken(1, "#004355".into()));
    }
}

