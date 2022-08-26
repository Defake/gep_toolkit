use std::fmt;
use std::ops::Range;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::operations::expressions::{Expression, RootExpression};
use crate::operations::op_set::{OperationSet, PrimitiveOperationSet};
use crate::operations::stack_op::StackOperation;

pub enum ExpressionTreeType {
    GEP,
    PGEP,
    RGEP,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct KExpressionParams {
    pub sub_length: u32,
    pub subs_count: u32,
    pub root_length: u32,
    pub roots_count: u32,

    // Can sub expressions contain other sub expressions (true) or only the Root can (false)
    pub reuse_sub_expr: bool,
}
impl KExpressionParams {
    pub fn default() -> KExpressionParams {
        KExpressionParams {
            sub_length: 0,
            subs_count: 0,
            root_length: 0,
            roots_count: 1,
            reuse_sub_expr: false,
        }
    }

    pub fn new(sub_length: u32, sub_count: u32, root_length: u32, root_count: u32, subs_in_subs: bool) -> KExpressionParams {
        KExpressionParams {
            sub_length,
            subs_count: sub_count,
            root_length,
            roots_count: root_count,
            reuse_sub_expr: subs_in_subs,
        }
    }
}

#[derive(Clone)]
pub struct KExpression {
    pub value: Vec<u32>,
    pub params: KExpressionParams,
    pub primitives_set: Arc<PrimitiveOperationSet>,
}

impl fmt::Display for KExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let mut operations_set = OperationSet::from_primitive_set(&self.primitives_set);
        let mut cursor = 0;

        let mut exprs_str = vec![];
        for _ in 0..self.params.subs_count {
            let mut expr_str = vec![];
            for _ in 0..self.params.sub_length {
                let operation = operations_set.operation_by_id(self.value[cursor]);
                expr_str.push(format!("{}", operation));
                cursor += 1;
            }
            exprs_str.push(format!("EXP[{}]", expr_str.join(", ")));

            // placeholder just to display an expression number
            operations_set.add_sub_expr(Expression::new(vec![]));
        }

        let mut roots_str = vec![];
        for _ in 0..self.params.roots_count {
            let mut root_str = vec![];
            for _ in 0..self.params.root_length {
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

                for _ in 0..self.params.subs_count {
                    let expr = self.expression_from_part(cursor..cursor + self.params.sub_length as usize, &operations_set);
                    operations_set.add_sub_expr(expr);

                    cursor += self.params.sub_length as usize;
                }

                let mut root_exprs: Vec<Expression> = Vec::new();
                for _ in 0..self.params.roots_count {
                    let expr = self.expression_from_part(cursor..cursor + self.params.root_length as usize, &operations_set);
                    root_exprs.push(expr);

                    cursor += self.params.root_length as usize;
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

        if self.params.reuse_sub_expr {
            let exp_included = index as u32 / self.params.sub_length;
            ids.add_exprs(exp_included);
        } else if index as u32 >= self.params.sub_length * self.params.subs_count {
            ids.add_exprs(self.params.subs_count);
        }

        self.value[index] = ids.random_id();
    }
}
