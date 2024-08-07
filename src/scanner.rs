pub struct Scanner{}

impl Scanner{
    pub fn new(_source: &str) -> Self {
        Self{}
    }

    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, String>{
        todo!();
    }
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Start,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Null, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub enum LiteralValue{
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierVal(String)
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: u64,
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: u64) -> Self {
        Self {
            token_type,
            lexeme,
            literal, 
            line_number
        }
    }
}