use crate::token;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn new_token(token_type: &str, ch: char) -> token::Token {
    if ch == '\0' {
        return token::Token {
            Type: String::from(token_type),
            Literal: String::from(""),
        };
    }
    token::Token {
        Type: String::from(token_type),
        Literal: ch.to_string(),
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let mut l = Lexer {
            input: code,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.as_bytes()[self.read_position] as char
        }
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

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = format!("{}{}", ch, self.ch);
                    tok = token::Token {
                        Type: String::from(token::EQ),
                        Literal: lit,
                    };
                } else {
                    tok = new_token(token::ASSIGN, self.ch);
                }
            }
            ';' => tok = new_token(token::SEMICOLON, self.ch),
            '(' => tok = new_token(token::LPAREN, self.ch),
            ')' => tok = new_token(token::RPAREN, self.ch),
            ',' => tok = new_token(token::COMMA, self.ch),
            '+' => tok = new_token(token::PLUS, self.ch),
            '{' => tok = new_token(token::LBRACE, self.ch),
            '}' => tok = new_token(token::RBRACE, self.ch),
            '-' => tok = new_token(token::MINUS, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = format!("{}{}", ch, self.ch);
                    tok = token::Token {
                        Type: String::from(token::NOT_EQ),
                        Literal: lit,
                    };
                } else {
                    tok = token::Token {
                        Type: String::from(token::BANG),
                        Literal: self.ch.to_string(),
                    };
                }
            }
            '*' => tok = new_token(token::ASTERISK, self.ch),
            '/' => tok = new_token(token::SLASH, self.ch),
            '>' => tok = new_token(token::GT, self.ch),
            '<' => tok = new_token(token::LT, self.ch),
            '\0' => tok = new_token(token::EOF, self.ch),
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    let toktype = token::lookup_ident(&lit);
                    tok = token::Token {
                        Type: String::from(toktype),
                        Literal: lit,
                    };
                    return tok;
                } else if is_digit(self.ch) {
                    let lit = self.read_number();
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
mod tests;
