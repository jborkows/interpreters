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
    pub(super) fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            parsing_errors: vec![],
        }
    }
    pub(super) fn empty() -> Self {
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
pub enum Expression {
    IdentifierExpression { name: String },
    LiteralInt { value: i32 },
}
impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::IdentifierExpression { name } => name.to_string(),
            Expression::LiteralInt { value } => value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement {
        token: Token,
        name: Box<Statement>,
        value: Box<Statement>,
    },
    ReturnStatement {
        token: Token,
        value: Box<Statement>,
    },
    ExpressionStatement {
        token: Token,
        expression: Box<Expression>,
    },
    IdentifierStatement {
        token: Token,
        name: String,
    },
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::LetStatement {
                token: _,
                name,
                value,
            } => {
                format!("let {} = {};", name.to_string(), value.to_string())
            }
            Statement::ReturnStatement { token: _, value } => {
                format!("return {};", value.to_string())
            }
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => expression.to_string(),
            Statement::IdentifierStatement { token: _, name } => name.to_string(),
        }
    }
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
impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
