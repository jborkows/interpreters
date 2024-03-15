use crate::lexers::{ColumnNumber, LineNumber, Token, TokenKind};

pub struct Program {
    statements: Vec<Statement>,
    parsing_errors: Vec<ParsingError>,
}

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    pub message: ParsingErrorKind,
    pub line: LineNumber,
    pub column: ColumnNumber,
}

#[derive(Debug, PartialEq)]
pub enum ParsingErrorKind {
    ExpectedIdentifier,
    ExpectedAssign(Option<TokenKind>),
    ExpectedInteger,
    UnexpectedToken(TokenKind),
    NotImplementedYet,
}

impl Program {
    pub(super) fn new() -> Self {
        Self {
            statements: vec![],
            parsing_errors: vec![],
        }
    }
}
impl IntoIterator for Program {
    type Item = Statement;
    type IntoIter = std::vec::IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.statements.into_iter().collect::<Vec<_>>().into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    IdentifierExpression {
        token: Token,
    },

    LetStatement {
        token: Token,
        name: Box<Statement>,
        value: Box<Statement>,
    },
    ReturnStatement {
        token: Token,
        value: Box<Statement>,
    },
    LiteralInt {
        token: Token,
        value: i32,
    },
}

impl Program {
    pub(super) fn push(&mut self, statement: Result<Statement, ParsingError>) {
        match statement {
            Ok(statement) => self.statements.push(statement),
            Err(error) => self.parsing_errors.push(error),
        }
    }
    pub fn errors(&self) -> &Vec<ParsingError> {
        self.parsing_errors.as_ref()
    }
}
