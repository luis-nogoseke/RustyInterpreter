use std::fmt;

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
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

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
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";

pub static KEYWORDS: phf::Map<&str, &str> = phf_map! {
    "fn" => FUNCTION,
    "let" => LET,
    "true" => TRUE,
    "false" => FALSE,
    "return" => RETURN,
    "if" => IF,
    "else" => ELSE
};

pub fn lookup_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(tok) => String::from(*tok),
        None => String::from(IDENT),
    }
}

impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Type: {} Literal: {}", self.Type, self.Literal)
    }
}
