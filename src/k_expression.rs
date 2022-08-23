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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let mut operations_set = OperationSet::from_primitive_set(&self.primitives_set);
        let mut cursor = 0;

        let mut exprs_str = vec![];
        for _ in 0..self.subs_count {
            let mut expr_str = vec![];
            for _ in 0..self.sub_length {
                let operation = operations_set.operation_by_id(self.value[cursor]);
                expr_str.push(format!("{}", operation));
                cursor += 1;
            }
            exprs_str.push(format!("EXP[{}]", expr_str.join(", ")));

            // placeholder just to display an expression number
            operations_set.add_sub_expr(Expression::new(vec![]));
        }

        let mut roots_str = vec![];
        for i in 0..self.roots_count {
            let mut root_str = vec![];
            for _ in 0..self.root_length {
                let operation = operations_set.operation_by_id(self.value[cursor]);
                root_str.push(format!("{}", operation));
                cursor += 1;
            }
            roots_str.push(format!("ROOT[{}]", root_str.join(", ")));
        }

        write!(f, "[{} - {}]", exprs_str.join(", "), roots_str.join(", "))
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
        let mut ids = self.primitives_set.operation_ids();

        let exp_included = index / self.sub_length as usize;
        ids.add_exprs(exp_included as u32);

        self.value[index] = ids.random_id();
    }
}
