use super::Parser;
use crate::ast::expression::InfixOperatorType;
use crate::ast::{
    expression::{Expression, PrefixOperatorType},
    statements::Statement,
};
use crate::tokens::TokenKind;
use crate::{join_collection, print_bash_error};
#[macro_export]
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
    assert_eq!(program.statements[0].to_string(), "let x=5");
    assert_eq!(program.statements[1].to_string(), "let y=10");
    assert_eq!(program.statements[2].to_string(), "let foobar=838383");
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
    assert_eq!(program.statements[0].to_string(), "return 5");
    assert_eq!(program.statements[1].to_string(), "return 10");
    assert_eq!(program.statements[2].to_string(), "return 838383");
}

#[test]
fn parse_identifier() {
    let input = r#"
    foobar;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => {
            check_if_identifiers_equals(expression, "foobar".to_string());
        }
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_number() {
    let input = r#"
    5;
    "#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => {
            check_if_integer_literal_equals(expression, 5);
        }
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_boolean() {
    let inputs = vec![("true;", true), ("false;", false)];
    for (input, value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::AExpression {
                token: _,
                expression,
            } => {
                check_if_boolean_literal_equals(expression, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn parse_string() {
    let inputs = vec![(r#""aaaa";"#, "aaaa".to_string())];
    for (input, value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::AExpression {
                token: _,
                expression,
            } => {
                check_if_strings_equals(expression, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn parse_prefix() {
    let inputs = vec![
        ("-5;", PrefixOperatorType::Minus, 5),
        ("!5;", PrefixOperatorType::Bang, 5),
    ];
    for (input, expected_operator, expected_value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::AExpression {
                token: _,
                expression,
            } => match expression {
                Expression::PrefixOperator {
                    token: _,
                    operator,
                    right,
                } => {
                    assert_eq!(operator, &expected_operator);
                    check_if_integer_literal_equals(right, expected_value);
                }
                _ => panic!("Expected PrefixOperator, got {:?}", expression),
            },
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

fn check_if_identifiers_equals(expression: &Expression, expected_value: String) {
    check_expression_value!(expression, Identifier, Identifier, expected_value);
}

fn check_if_strings_equals(expression: &Expression, expected_value: String) {
    check_expression_value!(expression, StringLiteral, StringLiteral, expected_value);
}

fn check_if_boolean_literal_equals(expression: &Expression, expected_value: bool) {
    match expression {
        Expression::BooleanLiteral { value, .. } => {
            if *value != expected_value {
                panic!(
                    "Expected BooleanLiteral with value {}, but got {}",
                    expected_value, value
                );
            }
        }
        _ => panic!("Expected BooleanLiteral, got {:?}", expression),
    }
}

fn check_if_integer_literal_equals(expression: &Expression, expected_value: u32) {
    check_expression_value!(expression, IntegerLiteral, Integer, expected_value);
}

#[test]
fn parse_infix_expression() {
    let inputs = vec![
        ("5 + 4;", InfixOperatorType::Plus, 5, 4),
        ("5 - 4;", InfixOperatorType::Minus, 5, 4),
        ("5 * 4;", InfixOperatorType::Multiply, 5, 4),
        ("5 / 4;", InfixOperatorType::Divide, 5, 4),
        ("5 == 4;", InfixOperatorType::Equal, 5, 4),
        ("5 < 4;", InfixOperatorType::LessThan, 5, 4),
        ("5 > 4;", InfixOperatorType::GreaterThan, 5, 4),
        ("5 != 4;", InfixOperatorType::NotEqual, 5, 4),
    ];
    for (input, expected_operator, expected_left, expected_right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::AExpression {
                token: _,
                expression,
            } => match expression {
                Expression::Infix {
                    token: _,
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(operator, &expected_operator);
                    check_if_integer_literal_equals(left, expected_left);
                    check_if_integer_literal_equals(right, expected_right);
                }
                _ => panic!("Expected InfixExpression, got {:?}", expression),
            },
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn parse_infix_expression_boolean() {
    let inputs = vec![
        ("true == true", InfixOperatorType::Equal, true, true),
        ("true != false", InfixOperatorType::NotEqual, true, false),
        ("false == false", InfixOperatorType::Equal, false, false),
        ("false != true", InfixOperatorType::NotEqual, false, true),
    ];
    for (input, expected_operator, expected_left, expected_right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::AExpression {
                token: _,
                expression,
            } => match expression {
                Expression::Infix {
                    token: _,
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(operator, &expected_operator);
                    check_if_boolean_literal_equals(left, expected_left);
                    check_if_boolean_literal_equals(right, expected_right);
                }
                _ => panic!("Expected InfixExpression, got {:?}", expression),
            },
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

#[test]
fn test_precedense_parsing() {
    let inputs = vec![
        ("-a * b", "((-a) * b)"),
        ("!a + b", "((!a) + b)"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c - d / e", "((a + (b * c)) - (d / e))"),
        ("a == b != c", "((a == b) != c)"),
        ("a == b < c", "(a == (b < c))"),
        ("a + (b + c) + d", "((a + (b + c)) + d)"),
        ("-(a + b)", "(-(a + b))"),
        ("a + add(b*c) + d", "((a + add((b * c))) + d)"),
        ("add(a*b[2],b[1])", "add((a * (b[2])), (b[1]))"),
        (
            "a * [1, 2, 3][b * c] *d",
            "((a * ([1, 2, 3][(b * c)])) * d)",
        ),
    ];
    for (input, expected) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0].to_string(), expected);
    }
}

#[test]
fn parse_if_condition() {
    let input = "if (x < y) { x } ";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::AIf {
                consequence,
                token,
                condition,
                alternative,
            } => {
                assert!(alternative.is_none());
                assert_eq!(token.kind, TokenKind::If);
                match condition.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::LessThan);
                        check_if_identifiers_equals(&left, "x".to_string());
                        check_if_identifiers_equals(&right, "y".to_string());
                    }
                    _ => panic!("Expected InfixExpression for condition"),
                }

                match consequence.as_ref() {
                    Statement::Block { statements, .. } => {
                        assert_eq!(statements.len(), 1);
                    }
                    _ => panic!("Expected BlockStatement in consequence"),
                }
            }
            _ => panic!("Expected IfExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_if_else_condition() {
    let input = "if (x < y) { x } else { y }";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::AIf {
                consequence,
                token,
                condition,
                alternative,
            } => {
                assert_eq!(token.kind, TokenKind::If);
                match condition.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::LessThan);
                        check_if_identifiers_equals(&left, "x".to_string());
                        check_if_identifiers_equals(&right, "y".to_string());
                    }
                    _ => panic!("Expected InfixExpression for condition"),
                }
                match alternative {
                    Some(alternative) => match alternative.as_ref() {
                        Statement::Block { statements, .. } => {
                            assert_eq!(statements.len(), 1);
                            match &statements[0] {
                                Statement::AExpression {
                                    token: _,
                                    expression,
                                } => {
                                    check_if_identifiers_equals(expression, "y".to_string());
                                }
                                _ => panic!("Expected ExpressionStatement in alternative"),
                            }
                        }
                        _ => panic!("Expected BlockStatement in alternative"),
                    },
                    None => panic!("Expected alternative to be Some"),
                }

                match consequence.as_ref() {
                    Statement::Block { statements, .. } => {
                        assert_eq!(statements.len(), 1);
                    }
                    _ => panic!("Expected BlockStatement in consequence"),
                }
            }
            _ => panic!("Expected IfExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_function_literal() {
    let input = "fn(x, y) { x + y; }";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => {
                assert_eq!(parameters.len(), 2);
                assert_eq!(parameters[0].to_string(), "x");
                assert_eq!(parameters[1].to_string(), "y");
                let block = match body.as_ref() {
                    Statement::Block { statements, .. } => &statements[0],
                    _ => panic!("Expected BlockStatement in function body"),
                };
                let expression = match block {
                    Statement::AExpression {
                        token: _,
                        expression,
                    } => expression,
                    _ => panic!("Expected ExpressionStatement in function body"),
                };
                match expression {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::Plus);
                        check_if_identifiers_equals(left, "x".to_string());
                        check_if_identifiers_equals(right, "y".to_string());
                    }
                    _ => panic!("Expected InfixExpression in function body"),
                }
            }
            _ => panic!("Expected FunctionLiteral, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_call_expression() {
    let input = "add(x + z, y);";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Call {
                token: _,
                function,
                arguments,
            } => {
                assert_eq!(function.to_string(), "add");
                assert_eq!(arguments.len(), 2);
                match &arguments[0] {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::Plus);
                        check_if_identifiers_equals(left, "x".to_string());
                        check_if_identifiers_equals(right, "z".to_string());
                    }
                    _ => panic!("Expected InfixExpression for first argument"),
                }
                check_if_identifiers_equals(&arguments[1], "y".to_string());
            }
            _ => panic!("Expected CallExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_call_expression_with_literals() {
    let input = "add(1, false);";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Call {
                token: _,
                function,
                arguments,
            } => {
                assert_eq!(function.to_string(), "add");
                assert_eq!(arguments.len(), 2);
                check_if_integer_literal_equals(&arguments[0], 1);
                check_if_boolean_literal_equals(&arguments[1], false);
            }
            _ => panic!("Expected CallExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_array_literal() {
    let input = "[1, 2*2, 3-1];";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::ArrayLiteral { token: _, elements } => {
                assert_eq!(elements.len(), 3);
                check_if_integer_literal_equals(&elements[0], 1);
                match &elements[1] {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::Multiply);
                        check_if_integer_literal_equals(left, 2);
                        check_if_integer_literal_equals(right, 2);
                    }
                    _ => panic!("Expected InfixExpression for second element"),
                }
                match &elements[2] {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::Minus);
                        check_if_integer_literal_equals(left, 3);
                        check_if_integer_literal_equals(right, 1);
                    }
                    _ => panic!("Expected InfixExpression for third element"),
                }
            }
            _ => panic!("Expected ArrayLiteral, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_index_expression() {
    let input = "myArray[1+1];";
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Index {
                token: _,
                array,
                index,
            } => {
                assert_eq!(array.to_string(), "myArray");
                match index.as_ref() {
                    Expression::Infix {
                        token: _,
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*operator, InfixOperatorType::Plus);
                        check_if_integer_literal_equals(left, 1);
                        check_if_integer_literal_equals(right, 1);
                    }
                    _ => panic!("Expected InfixExpression for index got {:?}", index),
                }
            }
            _ => panic!("Expected IndexExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_map_literal() {
    let input = r#"{"one":1, "two":2,"three":3+5}"#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::MapLiteral { token: _, elements } => {
                assert_eq!(elements.len(), 3);
                for (key, value) in elements {
                    match key {
                        Expression::StringLiteral(token) => match token.as_ref().kind {
                            TokenKind::StringLiteral(ref s) => match s.as_str() {
                                "one" => check_if_integer_literal_equals(value, 1),
                                "two" => check_if_integer_literal_equals(value, 2),
                                "three" => match value {
                                    Expression::Infix {
                                        token: _,
                                        left,
                                        operator,
                                        right,
                                    } => {
                                        assert_eq!(*operator, InfixOperatorType::Plus);
                                        check_if_integer_literal_equals(left, 3);
                                        check_if_integer_literal_equals(right, 5);
                                    }
                                    _ => panic!("Expected InfixExpression for value of 'three'"),
                                },
                                _ => panic!("Unexpected key: {}", s),
                            },
                            _ => panic!("Expected StringLiteral for key, got {:?}", key),
                        },
                        _ => panic!("Expected StringLiteral for key, got {:?}", key),
                    }
                }
            }
            _ => panic!("Expected MapLiteral, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_empty_map_literal() {
    let input = r#"{}"#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::MapLiteral { token: _, elements } => {
                assert_eq!(elements.len(), 0);
            }
            _ => panic!("Expected MapLiteral, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

#[test]
fn parse_getting_by_key() {
    let input = r#"{"one": 1, "two": 2}["one"]"#;
    let mut parser = Parser::from_string(input);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::AExpression {
            token: _,
            expression,
        } => match expression {
            Expression::Index {
                token: _,
                array,
                index,
            } => {
                match array.as_ref() {
                    Expression::MapLiteral { token: _, elements } => {
                        assert_eq!(elements.len(), 2);
                    }
                    _ => panic!("Expected MapLiteral for array, got {:?}", array),
                }
                check_if_strings_equals(index, "one".to_string());
            }
            _ => panic!("Expected IndexExpression, got {:?}", expression),
        },
        _ => panic!("Expected ExpressionStatement"),
    }
}

fn check_parser_errors(parser: &Parser) {
    if !parser.errors.is_empty() {
        panic!(
            "Parser errors: \n{}",
            print_bash_error!(join_collection!(&parser.errors, "\n"))
        );
    }
}
