use std::fmt;
use std::ops::Range;
use std::sync::Arc;

use super::expressions::Expression;
use crate::operation_set::OperationSet;
use crate::stack_operation::StackOperation;

use super::expressions::RootExpression;
use super::operation_set::PrimitiveOperationSet;

pub enum ExpressionTreeType {
    GEP,
    PGEP,
    RGEP,
}

#[derive(Clone)]
pub struct KExpression {
    pub value: Vec<u32>,
    pub sub_length: u32,
    pub subs_count: u32,
    pub root_length: u32,
    pub roots_count: u32,

    pub primitives_set: Arc<PrimitiveOperationSet>,
}

impl fmt::Display for KExpression {
    // TODO: readable representation of operations
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.value)
    }
}

impl KExpression {
    fn expression_from_part(&self, expr_range: Range<usize>, operations_set: &OperationSet) -> Expression {
        let k_expr_part = &self.value[expr_range];
        let mut expr_operations: Vec<StackOperation> = Vec::new();
        for operation_id in k_expr_part {
            let operation = operations_set.operation_by_id(*operation_id);
            expr_operations.push(operation);
        }

        Expression::new(expr_operations)
    }

    pub fn expression(&self, expr_type: ExpressionTreeType) -> RootExpression {
        match expr_type {
            ExpressionTreeType::GEP => {
                todo!()
            }

            ExpressionTreeType::PGEP => {
                todo!()
            }

            ExpressionTreeType::RGEP => {
                let mut cursor: usize = 0;
                let mut operations_set = OperationSet::from_primitive_set(&self.primitives_set);

                for _ in 0..self.subs_count {
                    let expr = self.expression_from_part(cursor..cursor + self.sub_length as usize, &operations_set);
                    operations_set.add_sub_expr(expr);

                    cursor += self.sub_length as usize;
                }

                let mut root_exprs: Vec<Expression> = Vec::new();
                for _ in 0..self.roots_count {
                    let expr = self.expression_from_part(cursor..cursor + self.root_length as usize, &operations_set);
                    root_exprs.push(expr);

                    cursor += self.root_length as usize;
                }

                RootExpression {
                    exprs: root_exprs,
                    args_count: self.primitives_set.args_count(),
                }
            }
        }
    }

    pub fn mutate(&mut self, index: usize) {
        // TODO: do it with regard of adfs position
        let ids = self.primitives_set.operation_ids();
        self.value[index] = ids.random_id();
    }
}
