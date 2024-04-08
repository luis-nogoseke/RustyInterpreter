use crate::token;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
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

    fn read_identifier(&mut self) {
        let p = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        &self.input[p..self.position];
    }

    fn next_token(&mut self) -> token::Token {
        let mut tok: token::Token;

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
                }
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
            '\0' => {
                tok = token::Token {
                    Type: String::from(token::EOF),
                    Literal: String::from(""),
                };
            }
            _ => {
                tok = token::Token {
                    Type: String::from(token::ILLEGAL),
                    Literal: String::from(""),
                };
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
        let input = String::from(";,+=");
        let tests: Vec<(token::TokenType, String)> = Vec::from([
            (String::from(token::SEMICOLON), String::from(";")),
            (String::from(token::COMMA), String::from(",")),
            (String::from(token::PLUS), String::from("+")),
            (String::from(token::ASSIGN), String::from("=")),
        ]);
        let mut l = Lexer::new(input);
        let mut tok: token::Token;
        for x in tests.iter() {
            tok = l.next_token();
            assert_eq!(x.0, tok.Type);
            assert_eq!(x.1, tok.Literal);
        }
    }
}
