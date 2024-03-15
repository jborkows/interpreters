use crate::lexers::ColumnNumber;
use crate::lexers::LineNumber;
use crate::lexers::Token;
use crate::lexers::TokenKind;
use crate::lexers::TokenKind::*;
use Statement::*;

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
    LiteralInt {
        token: Token,
        value: i32,
    },
}

impl Program {
    fn push(&mut self, statement: Result<Statement, ParsingError>) {
        match statement {
            Ok(statement) => self.statements.push(statement),
            Err(error) => self.parsing_errors.push(error),
        }
    }
    pub fn errors(&self) -> &Vec<ParsingError> {
        self.parsing_errors.as_ref()
    }
}

pub fn parse<T>(mut tokens: T) -> Program
where
    T: Iterator<Item = Token>,
{
    let mut program = Program {
        statements: vec![],
        parsing_errors: vec![],
    };

    while let Some(token) = tokens.next() {
        match token.kind() {
            Let() => {
                let statement = parse_let_statement(&mut tokens, token);
                program.push(statement);
            }
            Semicolon() => {}
            _ => program.push(Result::Err(ParsingError {
                message: ParsingErrorKind::UnexpectedToken(token.kind().clone()),
                line: token.line(),
                column: token.column(),
            })),
        }
    }
    program
}

fn parse_let_statement<T>(tokens: &mut T, token: Token) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let name = expect_indetifier(tokens, &token)?;
    match tokens.next() {
        Some(Token(_, _, Assign())) => {
            let value = parse_expression(tokens, &token)?;
            Ok(LetStatement {
                token,
                name: Box::new(name),
                value: Box::new(value),
            })
        }
        Some(Token(line, column, kind)) => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedAssign(Some(kind)),
            line: line,
            column: column,
        }),
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedAssign(None),
            line: token.line(),
            column: token.column(),
        }),
    }
}

fn expect_indetifier<T>(tokens: &mut T, parent_token: &Token) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(aaa) => {
            if let Identifier(_) = aaa.kind() {
            } else {
                return Result::Err(ParsingError {
                    message: ParsingErrorKind::ExpectedIdentifier,
                    line: aaa.line(),
                    column: aaa.column(),
                });
            }
            Ok(IdentifierExpression { token: aaa })
        }
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedIdentifier,
            line: parent_token.line(),
            column: parent_token.column(),
        }),
    }
}
fn parse_expression<'a, T>(tokens: &mut T, parent_token: &Token) -> Result<Statement, ParsingError>
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(Token(_, _, Integer(value))) => Ok(LiteralInt {
            token: token.unwrap(),
            value,
        }),
        Some(Token(line, column, _)) => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedInteger,
            line: line,
            column: column,
        }),
        _ => Result::Err(ParsingError {
            message: ParsingErrorKind::ExpectedInteger,
            line: parent_token.line(),
            column: parent_token.column(),
        }),
    }
}
