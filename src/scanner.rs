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
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, START,

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    INDENTIFIER, STRING, NUMBER,

    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}