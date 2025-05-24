use super::Parser;
use crate::ast::expression::{
    BooleanLiteral, CallExpression, FunctionLiteral, Identifier, IfExpression, InfixExpression,
    InfixOperatorType, StringLiteral,
};
use crate::ast::{
    expression::{Expression, IntegerLiteral, PrefixOperator, PrefixOperatorType},
    statements::Statement,
};
use crate::join_collection;

macro_rules! downcast_into {
    ($expr:expr, $target:ty) => {
        $expr.as_any().downcast_ref::<$target>().expect(concat!(
            "Expected ",
            stringify!($target),
            ""
        ))
    };
}

macro_rules! print_bash_error {
    ($msg:expr) => {
        format!("\x1b[31m{}\x1b[0m", $msg)
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
        Statement::ExpressionStatement {
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
        Statement::ExpressionStatement {
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
            Statement::ExpressionStatement {
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
            Statement::ExpressionStatement {
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
    for (input, operator, value) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let operator_expression = downcast_into!(expression, PrefixOperator);
                assert_eq!(operator_expression.operator, operator);
                check_if_integer_literal_equals(&operator_expression.right, value);
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}

fn check_if_identifiers_equals(expression: &Box<dyn Expression>, expected_value: String) {
    let literal = downcast_into!(expression, Identifier);
    if literal.value() != expected_value {
        panic!(
            "Expected Identifier with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
}

fn check_if_strings_equals(expression: &Box<dyn Expression>, expected_value: String) {
    let literal = downcast_into!(expression, StringLiteral);
    if literal.value() != expected_value {
        panic!(
            "Expected Identifier with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
}

fn check_if_boolean_literal_equals(expression: &Box<dyn Expression>, expected_value: bool) {
    let literal = downcast_into!(expression, BooleanLiteral);
    if literal.value != expected_value {
        panic!(
            "Expected Boolean literal with value {}, but got {}",
            expected_value, literal.value
        );
    }
}

fn check_if_integer_literal_equals(expression: &Box<dyn Expression>, expected_value: u32) {
    let literal = downcast_into!(expression, IntegerLiteral);
    if literal.value() != expected_value {
        panic!(
            "Expected IntegerLiteral with value {}, but got {}",
            expected_value,
            literal.value()
        );
    }
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
    for (input, operator, left, right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let infix = downcast_into!(expression, InfixExpression);
                assert_eq!(infix.operator, operator);
                check_if_integer_literal_equals(&infix.left, left);
                check_if_integer_literal_equals(&infix.right, right);
            }
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
    for (input, operator, left, right) in inputs {
        let mut parser = Parser::from_string(input);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                let infix = downcast_into!(expression, InfixExpression);
                assert_eq!(infix.operator, operator);
                check_if_boolean_literal_equals(&infix.left, left);
                check_if_boolean_literal_equals(&infix.right, right);
            }
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
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            let if_expression = downcast_into!(expression, IfExpression);
            let condition = downcast_into!(&if_expression.condition, InfixExpression);
            assert_eq!(condition.operator, InfixOperatorType::LessThan);
            check_if_identifiers_equals(&condition.left, "x".to_string());
            check_if_identifiers_equals(&condition.right, "y".to_string());
            let consequence = &if_expression.consequences()[0];
            match consequence {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => {
                    check_if_identifiers_equals(expression, "x".to_string());
                }
                _ => panic!("Expected ExpressionStatement in consequence"),
            }
        }
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
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            let if_expression = downcast_into!(expression, IfExpression);
            let condition = downcast_into!(&if_expression.condition, InfixExpression);
            assert_eq!(condition.operator, InfixOperatorType::LessThan);
            check_if_identifiers_equals(&condition.left, "x".to_string());
            check_if_identifiers_equals(&condition.right, "y".to_string());
            let consequence = &if_expression.consequences()[0];
            match consequence {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => {
                    check_if_identifiers_equals(expression, "x".to_string());
                }
                _ => panic!("Expected ExpressionStatement in consequence"),
            }
            let alternative = &if_expression.alternative().unwrap()[0];
            match alternative {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => {
                    check_if_identifiers_equals(expression, "y".to_string());
                }
                _ => panic!("Expected ExpressionStatement in alternative"),
            }
        }
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
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            let function_literal = downcast_into!(expression, FunctionLiteral);
            assert_eq!(function_literal.parameters.len(), 2);
            assert_eq!(function_literal.parameters[0].to_string(), "x");
            assert_eq!(function_literal.parameters[1].to_string(), "y");
            let block = match &function_literal.body {
                Statement::BlockStatement { statements, .. } => &statements[0],
                _ => panic!("Expected BlockStatement in function body"),
            };
            let expression = match block {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => expression,
                _ => panic!("Expected ExpressionStatement in function body"),
            };
            let addition = downcast_into!(expression, InfixExpression);

            assert_eq!(addition.operator, InfixOperatorType::Plus);
            check_if_identifiers_equals(&addition.left, "x".to_string());
            check_if_identifiers_equals(&addition.right, "y".to_string());
        }
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
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            let call_expression = downcast_into!(expression, CallExpression);
            let function_identifier = downcast_into!(&call_expression.function, Identifier);
            assert_eq!(function_identifier.value(), "add");
            assert_eq!(call_expression.arguments.len(), 2);
            let first_argument = downcast_into!(&call_expression.arguments[0], InfixExpression);
            assert_eq!(first_argument.operator, InfixOperatorType::Plus);
            check_if_identifiers_equals(&first_argument.left, "x".to_string());
            check_if_identifiers_equals(&first_argument.right, "z".to_string());
            let second_argument = downcast_into!(&call_expression.arguments[1], Identifier);
            assert_eq!(second_argument.value(), "y");
        }
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
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => {
            let call_expression = downcast_into!(expression, CallExpression);
            let function_identifier = downcast_into!(&call_expression.function, Identifier);
            assert_eq!(function_identifier.value(), "add");
            assert_eq!(call_expression.arguments.len(), 2);
            check_if_integer_literal_equals(&call_expression.arguments[0], 1);
            check_if_boolean_literal_equals(&call_expression.arguments[1], false);
        }
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
