use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

use super::expressions::Expression;
use super::primitives::{Argument, Constant, Modifier, Operator, PrimitiveOperation};

#[derive(Debug, Clone, PartialEq)]
pub enum StackOperation {
    Primitive(PrimitiveOperation),
    Expression(Rc<Expression>, usize),
}
impl fmt::Display for StackOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            StackOperation::Primitive(pr) => write!(f, "{}", pr),
            StackOperation::Expression(expr, index) => write!(f, "EXP[{}]", *index),
        }
    }
}

pub struct Stack<'a> {
    value: Vec<f64>,
    args: &'a Vec<f64>,
}

impl<'a> Stack<'a> {
    pub fn new(args: &'a Vec<f64>) -> Stack {
        Stack {
            value: vec![],
            args,
        }
    }

    fn push(&mut self, value: f64) {
        self.value.push(value);
    }

    fn get_arg(&self, index: usize) -> f64 {
        match self.args.get(index) {
            None => panic!("Can't find arg with index {} on Stack with {} args", index, self.args.len()),
            Some(value) => *value,
        }
    }

    pub fn pop(&mut self) -> f64 {
        self.value.pop().unwrap()
    }

    pub fn len(&self) -> usize { self.value.len() }

    pub fn result(&self) -> f64 {
        match self.value.last() {
            None => 0f64,
            Some(result) => *result,
        }
    }
}

// impl Clone for StackOperation {
//     fn clone(&self) -> Self {
//         match self {
//             StackOperation::Primitive(p) => StackOperation::Primitive(p.clone()),
//             StackOperation::Expression(expr) => StackOperation::Expression(Rc::clone(expr))
//         }
//     }
// }

impl StackOperation {
    pub fn update_stack(&self, stack: &mut Stack) {
        match self {
            StackOperation::Expression(expression, _) => {
                if stack.len() >= 2 {
                    let arg2 = stack.pop();
                    let arg1 = stack.pop();
                    let result = expression.compute_result(&vec![arg1, arg2]);

                    let result = if result.is_nan() {
                        0.0
                    } else if result.is_infinite() {
                        f64::MAX
                    } else {
                        result
                    };
                    stack.push(result);
                }
            }

            StackOperation::Primitive(prim_op) => {
                match prim_op {
                    PrimitiveOperation::Constant(cons) => {
                        stack.push(cons.value());
                    }

                    PrimitiveOperation::Argument(arg) => {
                        let Argument::Arg(index) = arg;
                        let arg_value = stack.get_arg(*index as usize);
                        stack.push(arg_value);
                    }

                    PrimitiveOperation::Modifier(modifier) => {
                        if stack.len() >= 1 {
                            let arg = stack.pop();
                            let result = modifier.compute(arg);

                            let result = if result.is_nan() {
                                0.0
                            } else if result.is_infinite() {
                                f64::MAX
                            } else {
                                result
                            };
                            stack.push(result);
                        }
                    }

                    PrimitiveOperation::Operator(op) => {
                        if stack.len() >= 2 {
                            let arg2 = stack.pop();
                            let arg1 = stack.pop();
                            let result = op.compute(arg1, arg2);

                            let result = if result.is_nan() {
                                0.0
                            } else if result.is_infinite() {
                                f64::MAX
                            } else {
                                result
                            };
                            stack.push(result);
                        }
                    }
                }
            }
        }
    }

    // pub fn is_pure_operation(&self) -> bool {
    //     match self {
    //         StackOperation::Constant(_) => true,
    //         StackOperation::Modifier(_) => true,
    //         StackOperation::Operator(_) => true,
    //         _ => false
    //     }
    // }

    pub fn construct(operation: impl StackOperationConstructor) -> StackOperation {
        operation.stack_operation()
    }
}


pub trait StackOperationConstructor {
    fn stack_operation(self) -> StackOperation;
}

impl StackOperationConstructor for Expression {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Expression(Rc::new(self), usize::MAX)
    }
}
impl StackOperationConstructor for PrimitiveOperation {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Primitive(self)
    }
}
impl StackOperationConstructor for Argument {
    fn stack_operation(self) -> StackOperation {
        PrimitiveOperation::Argument(self).stack_operation()
    }
}
impl StackOperationConstructor for Constant {
    fn stack_operation(self) -> StackOperation {
        PrimitiveOperation::Constant(self).stack_operation()
    }
}
impl StackOperationConstructor for Modifier {
    fn stack_operation(self) -> StackOperation {
        PrimitiveOperation::Modifier(self).stack_operation()
    }
}
impl StackOperationConstructor for Operator {
    fn stack_operation(self) -> StackOperation {
        PrimitiveOperation::Operator(self).stack_operation()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);

        Constant::C1
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.result(), 1_f64);
    }

    #[test]
    fn test_argument_changes_stack() {
        let args = vec![100_f64];
        let mut stack = Stack::new(&args);
        assert_eq!(stack.result(), 0_f64);

        Argument::Arg(0)
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.result(), 100_f64);
    }

    #[test]
    #[should_panic]
    fn test_stack_panics_given_invalid_argument_index() {
        let args = vec![100_f64];
        let mut stack = Stack::new(&args);

        Argument::Arg(1)
            .stack_operation()
            .update_stack(&mut stack);
    }

    #[test]
    fn test_modifier_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(6_f64);

        PrimitiveOperation::Modifier(Modifier::Sqr)
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.result(), 36_f64);
    }

    // #[test]
    // fn test_custom_modifier_changes_stack() {
    //     let args = vec![];
    //     let mut stack = Stack::new(&args);
    //     stack.push(2_f64);
    //
    //     PrimitiveOperation::Modifier(Modifier { func: |x| x * x * 3_f64 }, "ABC")
    //         .stack_operation()
    //         .update_stack(&mut stack);
    //
    //     assert_eq!(stack.result(), 12_f64);
    // }

    #[test]
    fn test_operator_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(2_f64);
        stack.push(3_f64);

        PrimitiveOperation::Operator(Operator::Multiply)
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.result(), 6_f64);
    }

    // TODO:
    // #[test]
    // fn test_custom_operator_changes_stack() {
    //     let args = vec![];
    //     let mut stack = Stack::new(&args);
    //     stack.push(2_f64);
    //     stack.push(3_f64);
    //
    //     PrimitiveOperation::Operator(Operator { func: |x, y| (x - y) * (x + y) }, "ABC")
    //         .stack_operation()
    //         .update_stack(&mut stack);
    //
    //     assert_eq!(stack.pop(), -5_f64);
    //     assert_eq!(stack.result(), 0_f64);
    // }

    #[test]
    fn test_expression_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(2_f64);
        stack.push(3_f64);


        // (x + 1)^2 - y
        // x = 2, y = 6
        Expression::new(vec![
            Argument::Arg(0).stack_operation(),
            Constant::C1.stack_operation(),
            Operator::Plus.stack_operation(),
            Modifier::Sqr.stack_operation(),
            Argument::Arg(1).stack_operation(),
            Operator::Minus.stack_operation(),
        ])
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), 6_f64);
        assert_eq!(stack.result(), 0_f64);
    }
}
