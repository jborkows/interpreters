use std::rc::Rc;

use crate::ast::{
    base::Node,
    expression::{self, Expression},
    statements::{Program, Statement},
};

macro_rules! modify_box_expression {
    ($node:expr, $fun:expr) => {{
        let x = modify(Rc::new((**$node).clone()), $fun)
            .as_any()
            .downcast_ref::<Expression>()
            .unwrap()
            .clone();
        Box::new(x)
    }};
}

macro_rules! modify_expression {
    ($node:expr, $fun:expr) => {{
        let modified = modify(Rc::new($node.clone()), $fun);
        modified
            .as_any()
            .downcast_ref::<Expression>()
            .unwrap()
            .clone()
    }};
}

macro_rules! modify_box_statement {
    ($node:expr, $fun:expr) => {{
        let x = modify(Rc::new((**$node).clone()), $fun)
            .as_any()
            .downcast_ref::<Statement>()
            .unwrap()
            .clone();
        Box::new(x)
    }};
}

// TODO add error handling instead of ignoring with unwrap
pub fn modify<'a, F>(node: Rc<dyn Node + 'a>, fun: F) -> Rc<dyn Node + 'a>
where
    F: Fn(Rc<dyn Node + 'a>) -> Rc<dyn Node + 'a> + Clone,
{
    let program = node.as_any().downcast_ref::<Program>();
    if let Some(program) = program {
        let statements = program
            .statements
            .iter()
            .filter_map(|s| {
                let modified = modify(Rc::new(s.clone()), fun.clone());
                let any_rc: Rc<dyn std::any::Any> = modified.clone();
                any_rc
                    .downcast::<Statement>()
                    .ok()
                    .map(|rc_stmt| (*rc_stmt).clone())
            })
            .collect::<Vec<_>>();

        let output = Program { statements };

        println!("Program finished with {:?}", output);
        return Rc::new(output);
    }

    let expression = node.as_any().downcast_ref::<Expression>();
    if let Some(expression) = expression {
        return match expression {
            Expression::Infix {
                token,
                left,
                operator,
                right,
            } => {
                let left_as_expression = modify_box_expression!(left, fun.clone());
                let right_as_expression = modify_box_expression!(right, fun);
                Rc::new(Expression::Infix {
                    token: token.clone(),
                    left: left_as_expression,
                    operator: operator.clone(),
                    right: right_as_expression,
                })
            }
            Expression::PrefixOperator {
                token,
                operator,
                right,
            } => Rc::new(Expression::PrefixOperator {
                token: token.clone(),
                operator: operator.clone(),
                right: modify_box_expression!(right, fun.clone()),
            }),
            Expression::Index {
                token,
                array,
                index,
            } => Rc::new(Expression::Index {
                token: token.clone(),
                array: modify_box_expression!(array, fun.clone()),
                index: modify_box_expression!(index, fun.clone()),
            }),
            Expression::AIf {
                token,
                condition,
                consequence,
                alternative,
            } => Rc::new(Expression::AIf {
                token: token.clone(),
                condition: modify_box_expression!(condition, fun.clone()),
                consequence: modify_box_statement!(consequence, fun.clone()),
                alternative: match alternative {
                    Some(v) => Some(modify_box_statement!(v, fun.clone())),
                    None => None,
                },
            }),
            Expression::FunctionLiteral {
                token,
                parameters,
                body,
            } => {
                let modified_parameter = parameters
                    .as_ref()
                    .into_iter()
                    .map(|s| modify_expression!(s, fun.clone()))
                    .collect::<Vec<_>>();
                Rc::new(Expression::FunctionLiteral {
                    token: token.clone(),
                    parameters: modified_parameter.into(),
                    body: modify_box_statement!(body, fun.clone()),
                })
            }
            Expression::ArrayLiteral { token, elements } => {
                let modified_elements = elements
                    .into_iter()
                    .map(|s| modify_expression!(s, fun.clone()))
                    .collect::<Vec<_>>();
                Rc::new(Expression::ArrayLiteral {
                    token: token.clone(),
                    elements: modified_elements,
                })
            }
            Expression::MapLiteral { token, elements } => {
                let modified_elements = elements
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            modify_expression!(k, fun.clone()),
                            modify_expression!(v, fun.clone()),
                        )
                    })
                    .collect::<Vec<_>>();
                Rc::new(Expression::MapLiteral {
                    token: token.clone(),
                    elements: modified_elements,
                })
            }
            _ => fun(node),
        };
    }

    let statement = node.as_any().downcast_ref::<Statement>();
    if let Some(statement) = statement {
        match statement {
            Statement::Block { token, statements } => {
                let modified_statements = statements
                    .as_ref()
                    .into_iter()
                    .map(|s| {
                        let modified = modify(Rc::new(s.clone()), fun.clone());
                        modified
                            .as_any()
                            .downcast_ref::<Statement>()
                            .unwrap()
                            .clone()
                    })
                    .collect::<Vec<_>>();
                return Rc::new(Statement::Block {
                    token: token.clone(),
                    statements: Rc::new(modified_statements),
                });
            }
            Statement::Return {
                token,
                return_value,
            } => {
                let expression = modify_expression!(return_value, fun);
                return Rc::new(Statement::Return {
                    token: token.clone(),
                    return_value: expression,
                });
            }
            Statement::Let { token, name, value } => {
                let expression = modify_expression!(value, fun);
                return Rc::new(Statement::Let {
                    token: token.clone(),
                    name: name.clone(),
                    value: expression,
                });
            }
            Statement::AExpression { token, expression } => {
                let expression_value = modify(Rc::new(expression.clone()), fun.clone());
                let should_be_expression = expression_value
                    .as_any()
                    .downcast_ref::<Expression>()
                    .unwrap();
                let modified = Statement::AExpression {
                    token: token.clone(),
                    expression: should_be_expression.clone(),
                };
                return Rc::new(modified);
            }
        }
    }

    fun(node)
}
