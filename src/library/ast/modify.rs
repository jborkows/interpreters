use std::{ops::Deref, rc::Rc};

use crate::ast::{
    base::Node,
    expression::Expression,
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

//FIXME: too much reflection and assumptions about return values
pub fn modify<'a>(
    node: Rc<dyn Node + 'a>,
    fun: fn(Rc<dyn Node + 'a>) -> Rc<dyn Node + 'a>,
) -> Rc<dyn Node + 'a> {
    let program = node.as_any().downcast_ref::<Program>();
    if let Some(program) = program {
        let statements = program
            .statements
            .iter()
            .filter_map(|s| {
                let modified = modify(Rc::new(s.clone()), fun);
                let any_rc: Rc<dyn std::any::Any> = modified.clone();
                any_rc
                    .downcast::<Statement>()
                    .ok()
                    .map(|rc_stmt| (*rc_stmt).clone())
            })
            .collect::<Vec<_>>();

        let output = Program { statements };

        return Rc::new(output);
    }

    let statement = node.as_any().downcast_ref::<Statement>();
    if let Some(statement) = statement {
        match statement {
            Statement::AExpression { token, expression } => {
                let expression_value = match expression {
                    Expression::Infix {
                        token,
                        left,
                        operator,
                        right,
                    } => {
                        let left_as_expression = modify_box_expression!(left, fun);
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
                        right: modify_box_expression!(right, fun),
                    }),
                    Expression::Index {
                        token,
                        array,
                        index,
                    } => Rc::new(Expression::Index {
                        token: token.clone(),
                        array: modify_box_expression!(array, fun),
                        index: modify_box_expression!(index, fun),
                    }),
                    _ => modify(Rc::new(expression.clone()), fun),
                };
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
            _ => {}
        }
    }

    fun(node)
}
