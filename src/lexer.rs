use crate::token;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
    token::Token {
        Type: token_type,
        Literal: ch.to_string(),
    }
}

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(code: String) -> Lexer {
        let mut l = Lexer {
            input: code,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let p = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        String::from(&self.input[p..self.position])
    }

    fn read_number(&mut self) -> String {
        let p = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }
        String::from(&self.input[p..self.position])
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> token::Token {
        let tok: token::Token;

        self.skip_whitespace();

        println!("Read char {}", self.ch);
        match self.ch {
            '=' => {
                tok = token::Token {
                    Type: String::from(token::ASSIGN),
                    Literal: self.ch.to_string(),
                };
            }
            ';' => {
                tok = token::Token {
                    Type: String::from(token::SEMICOLON),
                    Literal: self.ch.to_string(),
                };
            }
            '(' => {
                tok = token::Token {
                    Type: String::from(token::LPAREN),
                    Literal: self.ch.to_string(),
                };
            }
            ')' => {
                tok = token::Token {
                    Type: String::from(token::RPAREN),
                    Literal: self.ch.to_string(),
                };
            }
            ',' => {
                tok = token::Token {
                    Type: String::from(token::COMMA),
                    Literal: self.ch.to_string(),
                };
            }
            '+' => {
                tok = token::Token {
                    Type: String::from(token::PLUS),
                    Literal: self.ch.to_string(),
                };
            }
            '{' => {
                tok = token::Token {
                    Type: String::from(token::LBRACE),
                    Literal: self.ch.to_string(),
                };
            }
            '}' => {
                tok = token::Token {
                    Type: String::from(token::RBRACE),
                    Literal: self.ch.to_string(),
                };
            }
            '-' => {
                tok = token::Token {
                    Type: String::from(token::MINUS),
                    Literal: self.ch.to_string(),
                };
            }
            '!' => {
                tok = token::Token {
                    Type: String::from(token::BANG),
                    Literal: self.ch.to_string(),
                };
            }
            '*' => {
                tok = token::Token {
                    Type: String::from(token::ASTERISK),
                    Literal: self.ch.to_string(),
                };
            }
            '/' => {
                tok = token::Token {
                    Type: String::from(token::SLASH),
                    Literal: self.ch.to_string(),
                };
            }
            '>' => {
                tok = token::Token {
                    Type: String::from(token::GT),
                    Literal: self.ch.to_string(),
                };
            }
            '<' => {
                tok = token::Token {
                    Type: String::from(token::LT),
                    Literal: self.ch.to_string(),
                };
            }
            '\0' => {
                tok = token::Token {
                    Type: String::from(token::EOF),
                    Literal: String::from(""),
                };
            }
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    println!("Read identifier {}", lit);
                    let toktype = token::lookup_ident(&lit);
                    tok = token::Token {
                        Type: String::from(toktype),
                        Literal: lit,
                    };
                    return tok;
                } else if is_digit(self.ch) {
                    let lit = self.read_number();
                    println!("Read number {}", lit);
                    tok = token::Token {
                        Type: String::from(token::INT),
                        Literal: lit,
                    };
                    return tok;
                } else {
                    tok = token::Token {
                        Type: String::from(token::ILLEGAL),
                        Literal: String::from(self.ch),
                    };
                }
            }
        }
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5
5 < 10 > 5;
",
        );
        let tests: Vec<(token::TokenType, String)> = Vec::from([
            (String::from(token::LET), String::from("let")),
            (String::from(token::IDENT), String::from("five")),
            (String::from(token::ASSIGN), String::from("=")),
            (String::from(token::INT), String::from("5")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::LET), String::from("let")),
            (String::from(token::IDENT), String::from("ten")),
            (String::from(token::ASSIGN), String::from("=")),
            (String::from(token::INT), String::from("10")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::LET), String::from("let")),
            (String::from(token::IDENT), String::from("add")),
            (String::from(token::ASSIGN), String::from("=")),
            (String::from(token::FUNCTION), String::from("fn")),
            (String::from(token::LPAREN), String::from("(")),
            (String::from(token::IDENT), String::from("x")),
            (String::from(token::COMMA), String::from(",")),
            (String::from(token::IDENT), String::from("y")),
            (String::from(token::RPAREN), String::from(")")),
            (String::from(token::LBRACE), String::from("{")),
            (String::from(token::IDENT), String::from("x")),
            (String::from(token::PLUS), String::from("+")),
            (String::from(token::IDENT), String::from("y")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::RBRACE), String::from("}")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::LET), String::from("let")),
            (String::from(token::IDENT), String::from("result")),
            (String::from(token::ASSIGN), String::from("=")),
            (String::from(token::IDENT), String::from("add")),
            (String::from(token::LPAREN), String::from("(")),
            (String::from(token::IDENT), String::from("five")),
            (String::from(token::COMMA), String::from(",")),
            (String::from(token::IDENT), String::from("ten")),
            (String::from(token::RPAREN), String::from(")")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::BANG), String::from("!")),
            (String::from(token::MINUS), String::from("-")),
            (String::from(token::SLASH), String::from("/")),
            (String::from(token::ASTERISK), String::from("*")),
            (String::from(token::INT), String::from("5")),
            (String::from(token::INT), String::from("5")),
            (String::from(token::LT), String::from("<")),
            (String::from(token::INT), String::from("10")),
            (String::from(token::GT), String::from(">")),
            (String::from(token::INT), String::from("5")),
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::EOF), String::from("")),
        ]);
        let mut l = Lexer::new(input);
        let mut tok: token::Token;
        for (i, x) in tests.iter().enumerate() {
            tok = l.next_token();
            assert_eq!(x.0, tok.Type, "Failure in test {i}");
            assert_eq!(x.1, tok.Literal, "Failure in test {i}");
        }
    }
}
