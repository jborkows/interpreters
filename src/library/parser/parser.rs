use crate::lexers::Token;
use crate::lexers::TokenKind::*;
use Statement::*;

pub struct Program {
    statements: Vec<Statement>,
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

pub fn parse<T>(mut tokens: T) -> Program
where
    T: Iterator<Item = Token>,
{
    let mut statements = vec![];
    while let Some(token) = tokens.next() {
        match token.kind() {
            Let() => {
                statements.push(parse_let_statement(&mut tokens, token.clone()));
            }
            Semicolon() => {}
            _ => panic!("Unexpected token? {:?}", token.kind()),
        }
    }
    Program { statements }
}

fn parse_let_statement<T>(tokens: &mut T, token: Token) -> Statement
where
    T: Iterator<Item = Token>,
{
    let name = expect_indetifier(tokens, &token);
    if let Some(Token(_, _, Assign())) = tokens.next() {
        let value = parse_expression(tokens, &token);
        LetStatement {
            token,
            name: Box::new(name),
            value: Box::new(value),
        }
    } else {
        panic!("Expected assign at {:?}:{:?}", token.line(), token.column())
    }
}

fn expect_indetifier<T>(tokens: &mut T, parent_token: &Token) -> Statement
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(aaa) => IdentifierExpression {
            // token: (aaa).clone(),
            token: aaa,
        },
        _ => panic!(
            "Expected identifier at {:?}:{:?}",
            parent_token.line(),
            parent_token.column()
        ),
    }
}
fn parse_expression<'a, T>(tokens: &mut T, parent_token: &Token) -> Statement
where
    T: Iterator<Item = Token>,
{
    let token = tokens.next();
    match token {
        Some(Token(_, _, Integer(value))) => LiteralInt {
            token: token.unwrap(),
            value,
        },
        _ => panic!(
            "Expected integer at {:?}:{:?}",
            parent_token.line(),
            parent_token.column()
        ),
    }
}
