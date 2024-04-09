use phf::phf_map;

pub type TokenType = String;

pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const GT: &str = ">";
pub const LT: &str = "<";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

// Delimeters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// KEYWORDS
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

//pub static KEYWORDS: phf::Map<&str, &str> = phf::Map::from([("fn", FUNCTION), ("let", LET)]);
pub static KEYWORDS: phf::Map<&str, &str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
};

pub fn lookup_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(tok) => String::from(*tok),
        None => String::from(IDENT),
    }
}
