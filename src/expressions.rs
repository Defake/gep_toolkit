use std::fmt;
use std::rc::Rc;
use super::operations as op;

pub struct RootExpression {
    pub exprs: Vec<Expression>,
    pub args_amount: u32,
}

impl RootExpression {
    pub fn compute_result(&self, args: &Vec<f64>) -> Vec<f64> {
        if args.len() as u32 != self.args_amount {
            panic!("Expected {} amount of arguments, got: {}", self.args_amount, args.len())
        }

        self.exprs.iter()
            .map(|expr| expr.compute_result(&args))
            .collect()
    }
}

#[derive(Debug)]
pub struct Expression {
    pub operations: Vec<Rc<dyn StackOperation>>,
}

impl Expression {
    fn compute_result(&self, args: &Vec<f64>) -> f64 {
        let mut stack: Vec<f64> = vec![];
        for operation in &self.operations {
            operation.update_stack(&mut stack, args);
        }

        match stack.get(0) {
            None => 0f64,
            Some(result) => result.clone(),
        }
    }
}


pub trait StackOperation: fmt::Debug {
    // TODO: make stack a struct and put args there
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>);

    fn is_pure_operation(&self) -> bool {
        true
    }
}


impl StackOperation for op::Constant {
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>) {
        stack.push(self.value.clone() as f64);
    }
}

impl StackOperation for op::Argument {
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>) {
        let value = match args.get(self.index as usize) {
            None => 0f64,
            Some(value) => value.clone(),
        };

        stack.push(value);
    }

    fn is_pure_operation(&self) -> bool {
        false
    }
}

impl StackOperation for op::Modifier {
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>) {
        if stack.len() >= 1 {
            let arg = stack.pop().unwrap();
            let f = self.func;
            stack.push(f(arg));
        }
    }
}

impl StackOperation for op::Operator {
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>) {
        if stack.len() >= 2 {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            let f = self.func;
            stack.push(f(arg1, arg2));
        }
    }
}

impl StackOperation for Expression {
    fn update_stack(&self, stack: &mut Vec<f64>, args: &Vec<f64>) {
        if stack.len() >= 2 {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            let result = self.compute_result(&vec![arg1, arg2]);
            stack.push(result);
        }
    }

    fn is_pure_operation(&self) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::Argument;

    #[test]
    fn should_work_with_no_work() {
        let expr = RootExpression {
            exprs: vec![],
            args_amount: 0,
        };

        let result = expr.compute_result(&vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_got_less_arguments_than_expected() {
        let expr = RootExpression {
            exprs: vec![],
            args_amount: 2,
        };

        expr.compute_result(&vec![0f64]);
    }

    #[test]
    fn expression_calculated_correctly() {
        let args = vec![4_f64, 5_f64];

        let root_expr = {
            let sub_expr: Rc<dyn StackOperation> = Rc::new(
                Expression {
                    // (b^2) - a
                    operations: vec![Rc::new(Argument { index: 0 }),
                                     Rc::new(Argument { index: 1 }),
                                     Rc::new(op::MODIFIER_SQUARE),
                                     Rc::new(op::OPERATOR_MINUS)]
                });

            let expr1 = Expression {
                // (((5 * 5) + Q(4)) - 1) = 26
                operations: vec![Rc::new(op::CONST_1),
                                 Rc::new(Argument { index: 0 }),
                                 Rc::new(op::MODIFIER_SQRT),
                                 Rc::new(Argument { index: 1 }),
                                 Rc::new(Argument { index: 1 }),
                                 Rc::new(op::OPERATOR_MULTIPLY),
                                 Rc::new(op::OPERATOR_PLUS),
                                 Rc::new(op::OPERATOR_MINUS)]
            };

            let expr2 = Expression {
                // 100 * E(4, 5) = 2100
                operations: vec![Rc::new(Argument { index: 1 }),
                                 Rc::new(Argument { index: 0 }),
                                 Rc::clone(&sub_expr),
                                 Rc::new(op::CONST_100),
                                 Rc::new(op::OPERATOR_MULTIPLY)]
            };

            RootExpression {
                exprs: vec![expr1, expr2],
                args_amount: 2,
            }
        };

        let result = root_expr.compute_result(&args);
        assert_eq!(result, [26_f64, 2100_f64])
    }
}
