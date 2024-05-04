use crate::token;

trait NodeInt {}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    LetStatement,
}

pub trait Statement {
    fn statement_node(&self);
    fn token_literal(&self) -> String;
    fn st_type(&self) -> StatementType;
}

trait Expression {
    fn expression_node();
}

struct Node {
    text: String,
}

#[derive(Default, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    fn token_literal(&self) -> String {
        self.token.Literal.clone()
    }

    fn expression_node() {}
}

impl Expression for Identifier {
    fn expression_node() {}
}

#[derive(Default, Clone)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    //value: Expression,
}

impl Statement for LetStatement {
    fn token_literal(&self) -> String {
        self.token.Literal.clone()
    }

    fn statement_node(&self) {}

    fn st_type(&self) -> StatementType {
        StatementType::LetStatement
    }
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}
