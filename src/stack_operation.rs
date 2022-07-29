use std::any::Any;
use std::env::args;
use std::rc::Rc;

use crate::operation_set::PrimitiveOperationSet;
use crate::primitive_operations::{Argument, Constant, Modifier, Operator, PrimitiveOperation};

use super::expressions::Expression;
use super::primitive_operations as op;

#[derive(Debug, Clone, PartialEq)]
pub enum StackOperation {
    Primitive(op::PrimitiveOperation),
    Expression(Rc<Expression>),
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
            StackOperation::Expression(expression) => {
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
                        stack.push(cons.value);
                    }

                    PrimitiveOperation::Argument(arg) => {
                        let arg_value = stack.get_arg(arg.index as usize);
                        stack.push(arg_value);
                    }

                    PrimitiveOperation::Modifier(mod_f) => {
                        if stack.len() >= 1 {
                            let arg = stack.pop();
                            let f = mod_f.func;
                            let result = f(arg);

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

                    PrimitiveOperation::Operator(op_f) => {
                        if stack.len() >= 2 {
                            let arg2 = stack.pop();
                            let arg1 = stack.pop();
                            let f = op_f.func;
                            let result = f(arg1, arg2);

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

impl StackOperationConstructor for Constant {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Primitive(PrimitiveOperation::Constant(self))
    }
}

impl StackOperationConstructor for Argument {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Primitive(PrimitiveOperation::Argument(self))
    }
}

impl StackOperationConstructor for Modifier {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Primitive(PrimitiveOperation::Modifier(self))
    }
}

impl StackOperationConstructor for Operator {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Primitive(PrimitiveOperation::Operator(self))
    }
}

impl StackOperationConstructor for Expression {
    fn stack_operation(self) -> StackOperation {
        StackOperation::Expression(Rc::new(self))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);

        Constant { value: 1_f64 }
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), 1_f64);
        assert_eq!(stack.result(), 0_f64);
    }

    #[test]
    fn test_argument_changes_stack() {
        let args = vec![100_f64];
        let mut stack = Stack::new(&args);
        assert_eq!(stack.result(), 0_f64);

        Argument { index: 0 }
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), 100_f64);
        assert_eq!(stack.result(), 0_f64);
    }

    #[test]
    #[should_panic]
    fn test_stack_panics_given_invalid_argument_index() {
        let args = vec![100_f64];
        let mut stack = Stack::new(&args);

        Argument { index: 1 }
            .stack_operation()
            .update_stack(&mut stack);
    }

    #[test]
    fn test_modifier_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(2_f64);

        Modifier { func: |x| x * x * 3_f64 }
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), 12_f64);
        assert_eq!(stack.result(), 0_f64);
    }

    #[test]
    fn test_operator_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(2_f64);
        stack.push(3_f64);

        Operator { func: |x, y| (x - y) * (x + y) }
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), -5_f64);
        assert_eq!(stack.result(), 0_f64);
    }

    #[test]
    fn test_expression_changes_stack() {
        let args = vec![];
        let mut stack = Stack::new(&args);
        stack.push(2_f64);
        stack.push(3_f64);


        // (x + 1)^2 - y
        Expression::new(vec![
            Argument { index: 0 }.stack_operation(),
            op::CONST_1.stack_operation(),
            op::OPERATOR_PLUS.stack_operation(),
            op::MODIFIER_SQUARE.stack_operation(),
            Argument { index: 1 }.stack_operation(),
            op::OPERATOR_MINUS.stack_operation(),
        ])
            .stack_operation()
            .update_stack(&mut stack);

        assert_eq!(stack.pop(), 6_f64);
        assert_eq!(stack.result(), 0_f64);
    }
}
