use self::ast::StatementType;

use super::*;

#[test]
fn test_let_statements() {
    let input = String::from(
        "let x = 5;
let y = 10;
let foobar = 838383;
",
    );

    let l = lexer::Lexer::new(input);
    let mut p = Parser::new(l);

    let mut program: ast::Program = ast::Program::default();

    match p.parse_program() {
        None => assert!(false, "Failed to parse program!"),
        Some(pr) => program = pr,
    }

    assert_eq!(
        program.statements.len(),
        3,
        "Failure in test, wrong number of statements"
    );

    let tests = Vec::from(["x", "y", "foobar"]);

    for (i, x) in tests.iter().enumerate() {
        let stmt: &dyn Statement = &*program.statements[i];
        if !test_let_statement(&*stmt, x) {
            return;
        }
    }
}

fn test_let_statement(st: &dyn Statement, name: &str) -> bool {
    if st.token_literal() != "let" {
        println!("Expected let, got {}", st.token_literal());
        return false;
    }

    if st.st_type() != StatementType::LetStatement {
        println!("Expected LetStatment, got {:?}", st.st_type());
        return false;
    }

    return true;
}
