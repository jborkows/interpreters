use crate::lexers::{ColumnNumber, LineNumber, Token};

use super::{Program, Statement};

#[test]
fn check_display() {
    let program = Program::new(vec![Statement::LetStatement {
        token: Token(
            LineNumber(1),
            ColumnNumber(1),
            crate::lexers::TokenKind::Let(),
        ),
        name: Box::new(Statement::IdentifierStatement {
            name: String::from("myVar"),
            token: Token(
                LineNumber(1),
                ColumnNumber(5),
                crate::lexers::TokenKind::Identifier(String::from("x")),
            ),
        }),
        value: Box::new(Statement::IdentifierStatement {
            name: String::from("anotherVar"),
            token: Token(
                LineNumber(1),
                ColumnNumber(5),
                crate::lexers::TokenKind::Identifier(String::from("x")),
            ),
        }),
    }]);
    assert_eq!(program.to_string(), "let myVar = anotherVar;");
}
