use crate::token;

trait NodeInt {}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum StatementType {
    #[default]
    Default,
    LetStatement,
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
pub struct Statement {
    pub token: token::Token,
    pub name: Identifier,
    pub st_type: StatementType,
    //value: Expression,
}

impl Statement {
    pub fn token_literal(&self) -> String {
        self.token.Literal.clone()
    }

    pub fn statement_node(&self) {}

    pub fn new_let_statement() -> Statement {
        let mut s = Statement::default();
        s.st_type = StatementType::LetStatement;
        s
    }
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
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
