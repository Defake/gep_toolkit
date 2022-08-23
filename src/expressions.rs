use crate::stack_operation::{Stack, StackOperation};

pub struct RootExpression {
    pub exprs: Vec<Expression>,
    pub args_count: u32,
}

impl RootExpression {
    pub fn compute_result(&self, args: &Vec<f64>) -> Vec<f64> {
        if args.len() as u32 != self.args_count {
            panic!("Expected {} arguments, got: {}", self.args_count, args.len())
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

    pub fn compute_result(&self, args: &Vec<f64>) -> f64 {
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
    use crate::primitive_operations::{Argument, Constant, Modifier, Operator, PrimitiveOperation};
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
        let x = 4_f64;
        let y = 5_f64;
        let args = vec![x, y];

        let root_expr = {
            let sub_expr: StackOperation = StackOperation::Expression(
                Rc::new(
                    exp::Expression {
                        // (y^2) - x = 21
                        operations: vec![Argument::Arg(1).stack_operation(),
                                         Modifier::Sqr.stack_operation(),
                                         Argument::Arg(0).stack_operation(),
                                         Operator::Minus.stack_operation(),
                        ]
                    }
                ),
                0);

            let expr1 = exp::Expression {
                // 1 - (Q(x) + (y * y)) = -26
                operations: vec![Constant::C1.stack_operation(),
                                 Argument::Arg(0).stack_operation(),
                                 Modifier::Sqrt.stack_operation(),
                                 Argument::Arg(1).stack_operation(),
                                 Argument::Arg(1).stack_operation(),
                                 Operator::Multiply.stack_operation(),
                                 Operator::Plus.stack_operation(),
                                 Operator::Minus.stack_operation()]
            };

            let expr2 = exp::Expression {
                // 100 * E(x, y) = 2100
                operations: vec![Argument::Arg(0).stack_operation(),
                                 Argument::Arg(1).stack_operation(),
                                 sub_expr.clone(),
                                 Constant::C100.stack_operation(),
                                 Operator::Multiply.stack_operation()]
            };

            RootExpression {
                exprs: vec![expr1, expr2],
                args_count: 2,
            }
        };

        let result = root_expr.compute_result(&args);
        assert_eq!(result, [-26_f64, 2100_f64])
    }
}
