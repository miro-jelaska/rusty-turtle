#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    LeftBracketToken,
    RightBracketToken,

    NumberToken,
    ColorToken,

    SetColorCommandToken,
    ForwardCommandToken,
    BackwardCommandToken,
    TurnRightCommandToken,
    TurnLeftCommandToken,

    RepeatLoopToken,

    IllegalToken,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    LeftBracketToken(usize),
    RightBracketToken(usize),

    NumberToken(usize, f64),
    ColorToken(usize, String),

    SetColorCommandToken(usize),
    ForwardCommandToken(usize),
    BackwardCommandToken(usize),
    TurnRightCommandToken(usize),
    TurnLeftCommandToken(usize),

    RepeatLoopToken(usize),

    IllegalToken(usize, String),
}

impl Token {
    pub fn line(&self) -> usize {
        match self {
            Token::LeftBracketToken(line) => *line,
            Token::RightBracketToken(line) => *line,

            // TODO: The number may not be the same as input since.
            // Example:
            //    Input string: 0.6
            //    Output string: 0.6000000000000001
            Token::NumberToken(line, _) => *line,
            Token::ColorToken(line, _) => *line,

            Token::SetColorCommandToken(line) => *line,
            Token::ForwardCommandToken(line) => *line,
            Token::BackwardCommandToken(line) => *line,
            Token::TurnRightCommandToken(line) => *line ,
            Token::TurnLeftCommandToken(line) => *line,

            Token::RepeatLoopToken(line) => *line,

            Token::IllegalToken(line, _) => *line,
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::LeftBracketToken(_) => "[".to_string(),
            Token::RightBracketToken(_) => "]".to_string(),

            // TODO: The number may not be the same as input since.
            // Example:
            //    Input string: 0.6
            //    Output string: 0.6000000000000001
            Token::NumberToken(_, number) => number.to_string(),
            Token::ColorToken(_, number) => number.to_string(),

            Token::SetColorCommandToken(_) => "COLOR".to_string(),
            Token::ForwardCommandToken(_) => "FD".to_string(),
            Token::BackwardCommandToken(_) => "BK".to_string(),
            Token::TurnRightCommandToken(_) => "RT".to_string(),
            Token::TurnLeftCommandToken(_) => "LR".to_string(),

            Token::RepeatLoopToken(_) => "REPEAT".to_string(),

            Token::IllegalToken(_, illegal) => illegal.clone(),
        }
    }
    pub fn get_type(&self) -> TokenType{
        match self {
            Token::LeftBracketToken(_) => TokenType::LeftBracketToken,
            Token::RightBracketToken(_) => TokenType::RightBracketToken,

            Token::NumberToken(_, _) => TokenType::NumberToken,
            Token::ColorToken(_, _) => TokenType::ColorToken,

            Token::SetColorCommandToken(_) => TokenType::SetColorCommandToken,
            Token::ForwardCommandToken(_) => TokenType::ForwardCommandToken,
            Token::BackwardCommandToken(_) => TokenType::BackwardCommandToken,
            Token::TurnRightCommandToken(_) => TokenType::TurnRightCommandToken,
            Token::TurnLeftCommandToken(_) => TokenType::TurnLeftCommandToken,

            Token::RepeatLoopToken(_) => TokenType::RepeatLoopToken,

            Token::IllegalToken(_, _) => TokenType::IllegalToken,
        }
    }

    pub fn is_type(&self, token_type: TokenType) -> bool {
        self.get_type() == token_type
    }
}
