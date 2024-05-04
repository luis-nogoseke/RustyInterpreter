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

if (5 < 10) {
    return true;
} else {
    return false;
}
30 == 30;
56 != 84;
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
        (String::from(token::IF), String::from("if")),
        (String::from(token::LPAREN), String::from("(")),
        (String::from(token::INT), String::from("5")),
        (String::from(token::LT), String::from("<")),
        (String::from(token::INT), String::from("10")),
        (String::from(token::RPAREN), String::from(")")),
        (String::from(token::LBRACE), String::from("{")),
        (String::from(token::RETURN), String::from("return")),
        (String::from(token::TRUE), String::from("true")),
        (String::from(token::SEMICOLON), String::from(";")),
        (String::from(token::RBRACE), String::from("}")),
        (String::from(token::ELSE), String::from("else")),
        (String::from(token::LBRACE), String::from("{")),
        (String::from(token::RETURN), String::from("return")),
        (String::from(token::FALSE), String::from("false")),
        (String::from(token::SEMICOLON), String::from(";")),
        (String::from(token::RBRACE), String::from("}")),
        (String::from(token::INT), String::from("30")),
        (String::from(token::EQ), String::from("==")),
        (String::from(token::INT), String::from("30")),
        (String::from(token::SEMICOLON), String::from(";")),
        (String::from(token::INT), String::from("56")),
        (String::from(token::NOT_EQ), String::from("!=")),
        (String::from(token::INT), String::from("84")),
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
