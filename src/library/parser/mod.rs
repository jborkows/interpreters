#[cfg(test)]
mod parser_tests;

use std::{rc::Rc, thread::current};

use crate::{
    ast::{
        self,
        expression::{Expression, FunctionLiteral, Identifier, InfixOperatorType},
        statements::{Program, Statement},
    },
    lexers::Lexer,
    tokens::{PureTokenKind, Token, TokenKind},
};

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    current_token: Rc<Token>,
    peek_token: Option<Rc<Token>>,
}

impl Parser {
    pub fn from_string(source: &str) -> Self {
        let mut lexer = Lexer::new();
        for line in source.lines() {
            lexer.process(line);
        }
        Self::new(lexer)
    }

    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next();
        let peek = lexer.next();
        if let None = current {
            panic!("Lexer is empty");
        }
        let parser = Self {
            lexer,
            errors: Vec::new(),
            current_token: current.unwrap(),
            peek_token: peek,
        };
        parser
    }

    fn next_token(&mut self) {
        let next = self.peek_token.take().expect("No next token");
        self.peek_token = self.lexer.next();
        self.current_token = next;
    }

    fn save_next_token(&mut self) {
        if self.is_finished() {
            return;
        }
        self.next_token();
    }

    pub fn is_finished(&self) -> bool {
        self.peek_token.is_none()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };
        while !self.is_finished() {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.save_next_token();
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => return self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_token = self.current_token.clone();
        if !self.expect_peek_and_move_into(&PureTokenKind::Identifier) {
            return None;
        }

        let name = ast::expression::Identifier {
            token: self.current_token.clone(),
        };
        if !self.expect_peek_and_move_into(&PureTokenKind::Assign) {
            return None;
        }
        self.save_next_token();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            return None;
        }
        if self.peek_token_is(&PureTokenKind::Semicolon) {
            self.save_next_token();
        }
        return Some(Statement::Let {
            token: let_token,
            name,
            value: value.unwrap(),
        });
    }

    fn peek_token_is(&self, pure_token_kind: &PureTokenKind) -> bool {
        if let Some(peek) = &self.peek_token {
            let existing: PureTokenKind = (&peek.kind).into();
            return existing == *pure_token_kind;
        }
        return false;
    }
    fn current_token_is(&self, pure_token_kind: &PureTokenKind) -> bool {
        let existing: PureTokenKind = (&self.current_token.kind).into();
        return existing == *pure_token_kind;
    }

    fn expect_peek_and_move_into(&mut self, pure_token_kind: &PureTokenKind) -> bool {
        if self.peek_token_is(pure_token_kind) {
            self.next_token();
            return true;
        } else {
            self.peek_error(pure_token_kind);
            return false;
        }
    }

    fn peek_error(&mut self, expected: &PureTokenKind) {
        let error = format!(
            "Expected next token to be {:?}, got {:?} instead",
            expected, self.peek_token
        );
        self.errors.push(error);
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_token = self.current_token.clone();
        self.save_next_token();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            return None;
        }
        if self.peek_token_is(&PureTokenKind::Semicolon) {
            self.save_next_token();
        }
        return Some(Statement::Return {
            token: return_token,
            return_value: value.unwrap(),
        });
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression_token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest);
        if expression.is_none() {
            return None;
        }
        if self.peek_token_is(&PureTokenKind::Semicolon) {
            self.save_next_token();
        }
        return Some(Statement::ExpressionStatement {
            token: expression_token,
            expression: expression.unwrap(),
        });
    }

    fn parse_prefix(&mut self) -> Option<Box<dyn Expression>> {
        return match self.current_token.kind {
            TokenKind::Identifier(_) => {
                let identifier = ast::expression::Identifier {
                    token: self.current_token.clone(),
                };
                Some(Box::new(identifier))
            }
            TokenKind::Integer(_value) => {
                let integer = ast::expression::IntegerLiteral {
                    token: self.current_token.clone(),
                };
                Some(Box::new(integer))
            }
            TokenKind::Negation => self.parse_prefix_expression(),
            TokenKind::Minus => self.parse_prefix_expression(),
            TokenKind::True | TokenKind::False => self.parse_boolean(),
            TokenKind::StringLiteral(_) => self.parse_string_literal(),
            TokenKind::LeftParen => self.parse_grouped_expression(),
            TokenKind::If => self.parse_if_expression(),
            TokenKind::Function => self.parse_function_expression(),
            _ => None,
        };
    }

    fn infix(&mut self, left_exp: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        return match self.peek_token.as_ref().unwrap().kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Slash
            | TokenKind::Asterisk
            | TokenKind::Equal
            | TokenKind::Inequal
            | TokenKind::LessThen
            | TokenKind::GreaterThen => {
                self.save_next_token();
                self.parse_infix_expression(left_exp)
            }
            TokenKind::LeftParen => {
                self.save_next_token();
                self.parse_call_expression(left_exp)
            }
            _ => None,
        };
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Option<Box<dyn ast::expression::Expression>> {
        let mut maybe_prefix = self.parse_prefix();
        if let None = maybe_prefix {
            self.errors.push(format!(
                "No prefix parse function for {:?} found",
                self.current_token.kind
            ));
            return None;
        }
        let mut left_exp = maybe_prefix.take().expect("Prefix really not found");
        while !self.peek_token_is(&PureTokenKind::Semicolon)
            && !self.is_finished()
            && precedence < precedence_from(&self.peek_token.as_ref().unwrap())
        {
            let mut infix = self.infix(left_exp);
            if let None = infix {
                return None;
            }
            let infix_really = infix.take().expect("Infix really not found");
            left_exp = infix_really;
        }
        return Some(left_exp);
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let operator = match self.current_token.kind {
            TokenKind::Negation => ast::expression::PrefixOperatorType::Bang,
            TokenKind::Minus => ast::expression::PrefixOperatorType::Minus,
            _ => panic!("Unknown prefix operator"),
        };
        let current_token = self.current_token.clone();
        self.save_next_token();
        let right = self.parse_expression(Precedence::Prefix);
        return Some(Box::new(ast::expression::PrefixOperator {
            token: current_token,
            operator,
            right: right.unwrap(),
        }));
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        let precedence = precedence_from(current_token.as_ref());
        let operator = token_into_operator(current_token.as_ref())
            .take()
            .expect("Operator not found");
        self.save_next_token();
        return match self.parse_expression(precedence) {
            None => None,
            Some(right) => {
                return Some(Box::new(ast::expression::InfixExpression {
                    token: current_token,
                    left,
                    operator,
                    right,
                }));
            }
        };
    }

    fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
        let boolean = match self.current_token.kind {
            TokenKind::True => true,
            TokenKind::False => false,
            _ => panic!("Unknown boolean"),
        };
        let current_token = self.current_token.clone();
        return Some(Box::new(ast::expression::BooleanLiteral {
            token: current_token,
            value: boolean,
        }));
    }

    fn parse_string_literal(&self) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        return Some(Box::new(ast::expression::StringLiteral {
            token: current_token,
        }));
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.save_next_token();
        let expression = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek_and_move_into(&PureTokenKind::RightParen) {
            return None;
        }
        return Some(expression.unwrap());
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        if !self.expect_peek_and_move_into(&PureTokenKind::LeftParen) {
            return None;
        }
        self.save_next_token();
        let condition = self.parse_expression(Precedence::Lowest);
        if condition.is_none() {
            return None;
        }
        if !self.expect_peek_and_move_into(&PureTokenKind::RightParen) {
            return None;
        }
        if !self.expect_peek_and_move_into(&PureTokenKind::LeftBrace) {
            return None;
        }
        let consequence = self.parse_block_statement();
        let alternative = if self.peek_token_is(&PureTokenKind::Else) {
            self.save_next_token();
            if !self.expect_peek_and_move_into(&PureTokenKind::LeftBrace) {
                return None;
            }
            Some(self.parse_block_statement())
        } else {
            None
        };
        return Some(Box::new(ast::expression::IfExpression::new(
            current_token,
            condition.unwrap(),
            consequence,
            alternative,
        )));
    }

    fn parse_block_statement(&mut self) -> Statement {
        let current_token = self.current_token.clone();
        let mut statements = Vec::new();
        self.save_next_token();
        while !self.is_finished() && !self.current_token_is(&PureTokenKind::RightBrace) {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                statements.push(statement);
            }
            self.save_next_token();
        }
        return Statement::BlockStatement {
            token: current_token,
            statements: Rc::new(statements),
        };
    }

    fn parse_function_expression(&mut self) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        if !self.expect_peek_and_move_into(&PureTokenKind::LeftParen) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek_and_move_into(&PureTokenKind::RightParen) {
            self.errors
                .push("Expected right parenthesis after function parameters".to_string());
            return None;
        }
        if !self.expect_peek_and_move_into(&PureTokenKind::LeftBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        return Some(Box::new(FunctionLiteral {
            token: current_token,
            parameters: Rc::new(parameters),
            body,
        }));
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        if self.peek_token_is(&PureTokenKind::RightParen) {
            return vec![];
        }
        let mut arguments: Vec<Identifier> = vec![];
        while !(self.is_finished() || self.current_token_is(&PureTokenKind::RightParen)) {
            println!("Current token: {:?}", self.current_token.kind.to_string(),);
            if self.expect_peek_and_move_into(&PureTokenKind::Identifier) {
                let identifier = Identifier {
                    token: self.current_token.clone(),
                };
                arguments.push(identifier);
            } else {
                self.errors.push(format!(
                    "Expected identifier, got {:?}",
                    self.current_token.kind
                ));
            }
            if self.peek_token_is(&PureTokenKind::Comma) {
                self.save_next_token();
            } else if self.peek_token_is(&PureTokenKind::RightParen) {
                break;
            } else {
                self.errors.push(format!(
                    "Expected comma or right parenthesis, got {:?}",
                    self.peek_token.as_ref().unwrap().kind
                ));
                break;
            }
        }
        return arguments;
    }

    fn parse_call_expression(
        &mut self,
        left_exp: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let current_token = self.current_token.clone();
        let arguments = self.parse_call_arguments();
        return Some(Box::new(ast::expression::CallExpression {
            token: current_token,
            function: left_exp,
            arguments,
        }));
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Expression>> {
        if self.peek_token_is(&PureTokenKind::RightParen) {
            self.save_next_token();
            return vec![];
        }
        let mut arguments: Vec<Box<dyn Expression>> = vec![];
        self.save_next_token();
        arguments.push(
            self.parse_expression(Precedence::Lowest)
                .expect("Expected expression after left parenthesis"),
        );
        while self.peek_token_is(&PureTokenKind::Comma) {
            self.save_next_token();
            self.save_next_token();
            if let Some(argument) = self.parse_expression(Precedence::Lowest) {
                arguments.push(argument);
            } else {
                self.errors
                    .push("Expected expression after comma".to_string());
            }
        }
        if !self.expect_peek_and_move_into(&PureTokenKind::RightParen) {
            self.errors
                .push("Expected right parenthesis after call arguments".to_string());
        }
        return arguments;
    }
}

fn token_into_operator(token: &Token) -> Option<InfixOperatorType> {
    let pure_token_kind: PureTokenKind = (&token.kind).into();
    match pure_token_kind {
        PureTokenKind::Plus => Some(InfixOperatorType::Plus),
        PureTokenKind::Minus => Some(InfixOperatorType::Minus),
        PureTokenKind::Slash => Some(InfixOperatorType::Divide),
        PureTokenKind::Asterisk => Some(InfixOperatorType::Multiply),
        PureTokenKind::Equal => Some(InfixOperatorType::Equal),
        PureTokenKind::Inequal => Some(InfixOperatorType::NotEqual),
        PureTokenKind::LessThen => Some(InfixOperatorType::LessThan),
        PureTokenKind::GreaterThen => Some(InfixOperatorType::GreaterThan),
        _ => None,
    }
}

fn precedence_from(token: &Token) -> Precedence {
    let pure_token_kind: PureTokenKind = (&token.kind).into();
    match pure_token_kind {
        PureTokenKind::Plus => Precedence::Sum,
        PureTokenKind::Minus => Precedence::Sum,
        PureTokenKind::Slash => Precedence::Product,
        PureTokenKind::Asterisk => Precedence::Product,
        PureTokenKind::Equal => Precedence::Equals,
        PureTokenKind::Inequal => Precedence::Equals,
        PureTokenKind::LessThen => Precedence::LessThan,
        PureTokenKind::GreaterThen => Precedence::LessThan,
        PureTokenKind::LeftParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessThan,
    Sum,
    Product,
    Prefix,
    Call,
}

enum FailIfPrefixNotFound {
    Yes,
    No,
}
