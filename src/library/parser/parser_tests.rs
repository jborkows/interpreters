use super::Parser;
use crate::ast::Node;

#[test]
fn let_parsing() {
    let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3);
    assert_eq!(program.statements[0].token_literal(), "let x=5");
    assert_eq!(program.statements[1].token_literal(), "let y=10");
    assert_eq!(program.statements[2].token_literal(), "let foobar=838383");
}

#[test]
fn return_parsing() {
    let input = r#"
    return 5;
    return 10;
    return 838383;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3);
    assert_eq!(program.statements[0].token_literal(), "return 5");
    assert_eq!(program.statements[1].token_literal(), "return 10");
    assert_eq!(program.statements[2].token_literal(), "return 838383");
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!("Parser errors: {:?}", parser.errors);
    }
}
