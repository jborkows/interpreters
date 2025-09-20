use std::rc::Rc;

use crate::{
    ast::{
        base::Node,
        expression::{self, Expression, PrefixOperatorType},
        modify,
        statements::{Program, Statement},
    },
    tokens::{Token, TokenKind},
};

fn one() -> Expression {
    let token = Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(1),
    };
    return Expression::IntegerLiteral(Rc::new(token));
}

fn two() -> Expression {
    let token = Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(2),
    };
    return Expression::IntegerLiteral(Rc::new(token));
}

fn four() -> Expression {
    let token = Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(4),
    };
    return Expression::IntegerLiteral(Rc::new(token));
}

fn turn_one_into_two<'a>(node: Rc<dyn Node + 'a>) -> Rc<dyn Node + 'a> {
    let expression = node.as_any().downcast_ref::<Expression>();
    if let Some(expression) = expression {
        return match expression {
            Expression::IntegerLiteral(token) => match token.kind {
                crate::tokens::TokenKind::Integer(value) => match value == 1 {
                    true => Rc::new(two()),
                    false => node,
                },
                _ => node,
            },
            _ => node,
        };
    }
    return node;
}

#[test]
fn should_be_able_modify() {
    let token = Rc::new(Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(1),
    });
    let program = Program {
        statements: vec![Statement::AExpression {
            token: token.clone(),
            expression: one(),
        }],
    };
    let result = modify(Rc::new(program), turn_one_into_two);
    let output_program = result.as_any().downcast_ref::<Program>();
    let output = output_program.unwrap();
    assert_eq!(output.statements.len(), 1);
    let first_statement = output.statements[0].clone();
    match first_statement {
        Statement::AExpression {
            token: _,
            expression,
        } => {
            check_if_integer_literal_equals(&expression, 2);
        }
        _ => panic!("Expected Integer expression got {:?}", first_statement),
    }
}

#[test]
fn should_not_modify() {
    let token = Rc::new(Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(2),
    });
    let program = Program {
        statements: vec![Statement::AExpression {
            token: token.clone(),
            expression: two(),
        }],
    };
    let result = modify(Rc::new(program), turn_one_into_two);
    let output_program = result.as_any().downcast_ref::<Program>();
    let output = output_program.unwrap();
    assert_eq!(output.statements.len(), 1);
    let first_statement = output.statements[0].clone();
    match first_statement {
        Statement::AExpression {
            token: _,
            expression,
        } => {
            check_if_integer_literal_equals(&expression, 2);
        }
        _ => panic!("Expected Integer expression got {:?}", first_statement),
    }
}

macro_rules! should_traverse_infix_expresion {
    ($($name:ident: ($input_left:expr, $input_right:expr, $output_left:expr, $output_right:expr),)*) => {
        $(
            #[test]
            fn $name() {
     let token = Rc::new(Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(0),
    });
    let program = Program {
        statements: vec![Statement::AExpression {
            token: token.clone(),
            expression: Expression::Infix {
                token: token,
                left: Box::new($input_left),
                operator: expression::InfixOperatorType::Plus,
                right: Box::new($input_right),
            },
        }],
    };
    let result = modify(Rc::new(program), turn_one_into_two);
    let output_program = result.as_any().downcast_ref::<Program>();
    let output = output_program.unwrap();
    assert_eq!(output.statements.len(), 1);
    let first_statement = output.statements[0].clone();
    match first_statement {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Infix {
                token: _,
                left,
                operator: _,
                right,
            } => {
                check_if_integer_literal_equals(&left, $output_left);
                check_if_integer_literal_equals(&right, $output_right);
            }
            _ => panic!("Expected infix expression got {:?}", expression),
        },
        _ => panic!("Expected expression statement got {:?}", first_statement),
    }
            }
        )*
    };
}

should_traverse_infix_expresion! {
    should_modify_infix_left_branch: (one(), two(), 2, 2),
    should_modify_infix_right_branch: (two(), one(), 2, 2),
    should_modify_infix_both: (one(), one(), 2, 2),
    should_not_modify_infix_both: (four(), four(), 4, 4),
}

macro_rules! should_traverse_prefix_expression {
    ($($name:ident: ($input:expr,  $output:expr),)*) => {
        $(
            #[test]
            fn $name() {
     let token = Rc::new(Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(0),
    });
    let program = Program {
        statements: vec![Statement::AExpression {
            token: token.clone(),
            expression: Expression::PrefixOperator {
                token: token,
                operator: PrefixOperatorType::Minus,
                right: Box::new($input),
            },
        }],
    };
    let result = modify(Rc::new(program), turn_one_into_two);
    let output_program = result.as_any().downcast_ref::<Program>();
    let output = output_program.unwrap();
    assert_eq!(output.statements.len(), 1);
    let first_statement = output.statements[0].clone();
    match first_statement {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::PrefixOperator {
                token: _,
                operator: _,
                right,
            } => {
                check_if_integer_literal_equals(&right, $output);
            }
            _ => panic!("Expected prefix expression got {:?}", expression),
        },
        _ => panic!("Expected expression statement got {:?}", first_statement),
    }
            }
        )*
    };
}
should_traverse_prefix_expression! {
    should_modify_prefix_expression:(one(), 2),
    should_not_modify_prefix_expression:(four(),4),
}

macro_rules! should_traverse_index_expression {
    ($($name:ident: ($left:expr, $index:expr,  $output_left:expr, $output_index:expr),)*) => {
        $(
            #[test]
            fn $name() {
     let token = Rc::new(Token {
        context: Option::None,
        kind: crate::tokens::TokenKind::Integer(0),
    });
    let program = Program {
        statements: vec![Statement::AExpression {
            token: token.clone(),
            expression: Expression::Index {
                token: token,
                array: Box::new($left),
                index: Box::new($index),
            },
        }],
    };
    let result = modify(Rc::new(program), turn_one_into_two);
    let output_program = result.as_any().downcast_ref::<Program>();
    let output = output_program.unwrap();
    assert_eq!(output.statements.len(), 1);
    let first_statement = output.statements[0].clone();
    match first_statement {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Index {
                token: _,
                array,
                index,
            } => {
                check_if_integer_literal_equals(&array, $output_left);
                check_if_integer_literal_equals(&index, $output_index);
            }
            _ => panic!("Expected index expression got {:?}", expression),
        },
        _ => panic!("Expected expression statement got {:?}", first_statement),
    }
            }
        )*
    };
}
should_traverse_index_expression! {
    should_modify_index_part_of_index_expression:(one(), four(), 2,4),
    should_modify_array_part_of_index_expression:(four(), one(), 4,2),
    should_modify_both_parts_of_index_expression:(one(), one(), 2,2),
    should_not_modify_both_parts_of_index_expression:(four(), four(), 4,4),
}

macro_rules! check_expression_value {
    ($expression:expr, $variant:ident, $token_kind:ident, $expected:expr) => {
        match $expression {
            Expression::$variant(inner) => match inner.as_ref().kind {
                TokenKind::$token_kind(ref value) => {
                    if value != &$expected {
                        panic!(
                            concat!(
                                "Expected ",
                                stringify!($token_kind),
                                " with value {}, but got {}"
                            ),
                            $expected, value
                        );
                    }
                }
                _ => panic!(
                    concat!("Expected ", stringify!($token_kind), ", got {:?}"),
                    inner
                ),
            },
            _ => panic!(
                concat!("Expected ", stringify!($variant), ", got {:?}"),
                $expression
            ),
        }
    };
}

fn check_if_integer_literal_equals(expression: &Expression, expected_value: u32) {
    check_expression_value!(expression, IntegerLiteral, Integer, expected_value);
}
