use crate::{
    ast::{self, Statement},
    lexer,
    token::{self, EOF},
};

pub struct Parser {
    lex: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Self {
        let mut p = Self {
            lex: l,
            cur_token: token::Token::new(),
            peek_token: token::Token::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    fn is_curr_token(&self, t: token::TokenType) -> bool {
        self.cur_token.Type == t
    }

    fn is_peek_token(&self, t: token::TokenType) -> bool {
        self.peek_token.Type == t
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.is_peek_token(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        println!("Parsing a let statement");
        let mut st = ast::LetStatement::default();
        st.token = self.cur_token.clone();
        println!("{:?}", st.token);
        if !self.expect_peek(String::from(token::IDENT)) {
            println!("NO IDENT");
            return None;
        }

        st.name = ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.Literal.clone(),
        };

        if !self.expect_peek(String::from(token::ASSIGN)) {
            println!("No ASSIGN");
            return None;
        }

        while !self.is_curr_token(String::from(token::SEMICOLON)) {
            self.next_token();
        }
        Some(st)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.Type.as_str() {
            token::LET => match self.parse_let_statement() {
                Some(l) => Some(Box::new(l)),
                None => None,
            },
            _ => None,
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut p = ast::Program::default();
        while self.cur_token.Type != token::EOF {
            println!("Will parse a statement");
            match self.parse_statement() {
                None => {
                    println!("Failed to parse statement!");
                    return None;
                }
                Some(st) => p.statements.push(st),
            }
            println!("Statement ok!");
            self.next_token();
        }
        Some(p)
    }
}

#[cfg(test)]
mod tests;
