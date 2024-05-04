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
        let stmt: &Statement = &program.statements[i];
        test_let_statement(&*stmt, x);
    }
}

fn test_let_statement(st: &Statement, name: &str) {
    if st.token_literal() != "let" {
        println!("Expected let, got {}", st.token_literal());
    }

    assert_eq!("let", st.token_literal());

    if st.st_type != StatementType::LetStatement {
        println!("Expected LetStatment, got {:?}", st.st_type);
    }
    assert_eq!(StatementType::LetStatement, st.st_type);

    if st.name.value != name {
        println!("Expected {} got {}", st.name.value, name);
    }
    assert_eq!(name, st.name.value);
    assert_eq!(name, st.name.token.Literal);
}
