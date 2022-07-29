use std::fmt;

use crate::stack_operation::{Stack, StackOperation};

pub struct RootExpression {
    pub exprs: Vec<Expression>,
    pub args_count: u32,
}

impl RootExpression {
    pub fn compute_result<'a>(&self, args: &'a Vec<f64>) -> Vec<f64> {
        if args.len() as u32 != self.args_count {
            panic!("Expected {} amount of arguments, got: {}", self.args_count, args.len())
        }

        self.exprs.iter()
            .map(|expr| expr.compute_result(&args))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    operations: Vec<StackOperation>,
}

impl Expression {
    pub fn new(operations: Vec<StackOperation>) -> Expression {
        Expression { operations }
    }

    pub fn compute_result<'a>(&self, args: &'a Vec<f64>) -> f64 {
        let mut stack = Stack::new(args);
        for operation in &self.operations {
            operation.update_stack(&mut stack);
        }

        stack.result()
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::expressions as exp;
    use crate::primitive_operations::*;
    use crate::primitive_operations as pr;
    use crate::stack_operation::StackOperation::*;
    use crate::stack_operation::StackOperationConstructor;

    use super::*;
    use super::super::primitive_operations as op;

    #[test]
    fn should_work_with_no_calculations() {
        let expr = RootExpression {
            exprs: vec![],
            args_count: 0,
        };

        let result = expr.compute_result(&vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_got_less_arguments_than_expected() {
        let expr = RootExpression {
            exprs: vec![],
            args_count: 2,
        };

        expr.compute_result(&vec![0f64]);
    }

    #[test]
    fn expression_calculated_correctly() {
        let args = vec![4_f64, 5_f64];

        let root_expr = {
            let sub_expr: StackOperation = StackOperation::Expression(
                Rc::new(
                    exp::Expression {
                        // (b^2) -a
                        operations: vec![op::Argument { index: 0 }.stack_operation(),
                                         op::Argument { index: 1 }.stack_operation(),
                                         op::MODIFIER_SQUARE.stack_operation(),
                                         op::OPERATOR_MINUS.stack_operation()]
                    }
                ));

            let expr1 = exp::Expression {
                // (((5 * 5) + Q(4)) - 1) = 26
                operations: vec![op::CONST_1.stack_operation(),
                                 Argument { index: 0 }.stack_operation(),
                                 op::MODIFIER_SQRT.stack_operation(),
                                 Argument { index: 1 }.stack_operation(),
                                 Argument { index: 1 }.stack_operation(),
                                 op::OPERATOR_MULTIPLY.stack_operation(),
                                 op::OPERATOR_PLUS.stack_operation(),
                                 op::OPERATOR_MINUS.stack_operation()]
            };

            let expr2 = exp::Expression {
                // 100 * E(4, 5) = 2100
                operations: vec![Argument { index: 0 }.stack_operation(),
                                 Argument { index: 1 }.stack_operation(),
                                 sub_expr.clone(),
                                 op::CONST_100.stack_operation(),
                                 op::OPERATOR_MULTIPLY.stack_operation()]
            };

            RootExpression {
                exprs: vec![expr1, expr2],
                args_count: 2,
            }
        };

        let result = root_expr.compute_result(&args);
        assert_eq!(result, [26_f64, 2100_f64])
    }
}
