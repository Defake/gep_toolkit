use std::{error, io};
use std::fmt::format;
use std::rc::Rc;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use simple_error::SimpleError;

use crate::utils::filesystem as fs;

use super::expressions::Expression;
use super::op_ids::OperationIds;
use super::primitives::{Argument, PrimitiveOperation};
use super::stack_op::StackOperation;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrimitiveOperationSet {
    operations_set: Vec<PrimitiveOperation>,
    args_count: u32,
}

impl PrimitiveOperationSet {
    pub fn new(mut operations: Vec<PrimitiveOperation>, args_count: u32) -> PrimitiveOperationSet {
        for op in &operations {
            match op {
                PrimitiveOperation::Argument(_) => {
                    panic!("Operation {:?} can not be passed as a custom operation for Genes Set", op)
                }
                _ => {}
            }
        }

        for arg_i in 0..args_count {
            operations.push(PrimitiveOperation::Argument(Argument::Arg(arg_i)));
        }

        PrimitiveOperationSet {
            operations_set: operations,
            args_count,
        }
    }

    pub fn args_count(&self) -> u32 { self.args_count }

    pub fn deserialize_from_file(&self, filename: &str)
        -> Result<PrimitiveOperationSet, Box<dyn error::Error>> {

        let ops_set: PrimitiveOperationSet = fs::deserialize_from_file(filename)?;
        Ok(ops_set)
    }

    pub fn serialize_to_file(&self, filename: &str) -> std::io::Result<()> {
        fs::serialize_to_file(filename, &self)
    }

    pub fn operation_ids(&self) -> OperationIds {
        OperationIds::new(self.operations_set.len() as u32)
    }
}


pub struct OperationSet {
    // operations: Arc<StackOperation>,
    primitive_set: Arc<PrimitiveOperationSet>,
    sub_expr_set: Vec<Rc<Expression>>,
}

impl OperationSet {
    pub fn from_primitive_set(primitives_set: &Arc<PrimitiveOperationSet>) -> OperationSet {
        // OperationSet {
        //     operations: Arc::clone(primitives_set) as Arc<StackOperation>,
        // }
        OperationSet {
            primitive_set: Arc::clone(primitives_set),
            sub_expr_set: vec![],
        }
    }

    // pub fn operation_by_id(&self, id: u32) -> StackOperation {
    //     // id is an index in OperationSet's default implementation, so we can just use vector indexing
    //     let operations_count = self.operations.len();
    //     let id = id as usize;
    //
    //     if id >= operations_count {
    //         panic!("ID {} is not available in OperationSet with {} operations",
    //                id, operations_count);
    //     }
    //
    //     self.operations[id].clone();
    // }

    pub fn operation_by_id(&self, id: u32) -> StackOperation {
        // id is an index in OperationSet's default implementation, so we can just use vector indexing
        let index = id as usize;
        let operations_count = self.primitive_set.operations_set.len();

        if index < operations_count {
            // TODO: Arc is slow for get- operations? Do thread-local performance optimizations
            return StackOperation::Primitive(self.primitive_set.operations_set[index].clone());
        }

        let sub_expr_index = index - operations_count;
        let subs_expr_count = self.sub_expr_set.len();

        if sub_expr_index >= subs_expr_count {
            panic!("ID {} is not available in OperationsDictionary with {} operations and {} sub expressions",
                   index, operations_count, subs_expr_count);
        }

        StackOperation::Expression(Rc::clone(&self.sub_expr_set[sub_expr_index as usize]), sub_expr_index)
    }

    pub fn add_sub_expr(&mut self, sub_expr: Expression) {
        self.sub_expr_set.push(Rc::new(sub_expr));
    }
}


#[cfg(test)]
mod tests {
    use crate::operations::primitives::{Constant, Modifier, Operator};
    use crate::operations::primitives::PrimitiveOperation;
    use crate::operations::stack_op::StackOperationConstructor;

    use super::*;

    #[test]
    fn should_generate_correct_number_of_args() {
        let ops_set = PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Constant(Constant::C1),
            PrimitiveOperation::Modifier(Modifier::Sqr),
            PrimitiveOperation::Operator(Operator::Plus),
        ], 2);

        assert_eq!(ops_set.args_count(), 2);

        assert_eq!(ops_set.operations_set.len(), 5);
        assert_eq!(ops_set.operations_set[3], PrimitiveOperation::Argument(Argument::Arg(0)));
        assert_eq!(ops_set.operations_set[4], PrimitiveOperation::Argument(Argument::Arg(1)));
    }

    #[test]
    fn should_create_operation_ids() {
        let ops_set = PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Constant(Constant::C1),
            PrimitiveOperation::Modifier(Modifier::Sqr),
            PrimitiveOperation::Operator(Operator::Plus),
        ], 2);

        let ids = ops_set.operation_ids();
        let max = *ids.random_ids(10000).iter().max().unwrap();
        assert_eq!(max, 4)
    }

    #[test]
    #[should_panic]
    fn should_panic_when_passed_arguments_to_constructor() {
        PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Argument(Argument::Arg(0)),
        ], 1);
    }

    #[test]
    fn should_return_correct_primitives_by_indexes() {
        let primitive_set = Arc::new(PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Constant(Constant::C1),
            PrimitiveOperation::Modifier(Modifier::Sqr),
            PrimitiveOperation::Operator(Operator::Plus),
        ], 2));
        let ops_set = OperationSet::from_primitive_set(&primitive_set);

        assert_eq!(ops_set.operation_by_id(0), Constant::C1.stack_operation());
        assert_eq!(ops_set.operation_by_id(1), Modifier::Sqr.stack_operation());
        assert_eq!(ops_set.operation_by_id(2), Operator::Plus.stack_operation());
        assert_eq!(ops_set.operation_by_id(3), Argument::Arg(0).stack_operation());
        assert_eq!(ops_set.operation_by_id(4), Argument::Arg(1).stack_operation());
    }

    #[test]
    #[should_panic]
    fn should_panic_when_operation_id_not_found() {
        let primitive_set = Arc::new(PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Constant(Constant::C1),
        ], 1));
        let ops_set = OperationSet::from_primitive_set(&primitive_set);

        ops_set.operation_by_id(2);
    }

    #[test]
    fn should_return_correct_expressions_by_indexes() {
        let primitive_set = Arc::new(PrimitiveOperationSet::new(vec![
            PrimitiveOperation::Constant(Constant::C1),
            PrimitiveOperation::Modifier(Modifier::Sqr),
        ], 1));
        let mut ops_set = OperationSet::from_primitive_set(&primitive_set);

        let expr1 = Expression::new(vec![
            Constant::C100.stack_operation(),
            Modifier::Sigmoid.stack_operation(),
        ]);
        ops_set.add_sub_expr(expr1.clone());

        let expr2 = Expression::new(vec![
            Argument::Arg(0).stack_operation(),
            Argument::Arg(1).stack_operation(),
            Operator::Plus.stack_operation(),
        ]);
        ops_set.add_sub_expr(expr2.clone());

        assert_eq!(ops_set.operation_by_id(3), StackOperation::Expression(Rc::new(expr1), 0));
        assert_eq!(ops_set.operation_by_id(4), StackOperation::Expression(Rc::new(expr2), 1));
    }

    #[test]
    #[should_panic]
    fn should_panic_when_expression_id_not_found() {
        let primitive_set = Arc::new(PrimitiveOperationSet::new(vec![], 1));
        let mut ops_set = OperationSet::from_primitive_set(&primitive_set);

        let expr1 = Expression::new(vec![]);
        ops_set.add_sub_expr(expr1.clone());

        ops_set.operation_by_id(2);
    }
}
