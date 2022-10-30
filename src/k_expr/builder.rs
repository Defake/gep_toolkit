use std::sync::Arc;

use crate::operations::op_set::*;

use super::core::{KExpression, KExpressionParams};

#[derive(Clone)]
pub struct KExpressions {
    pub params: KExpressionParams,
    // TODO: make an option to ensure the tree is always full width
    // full_width: bool,
    pub operations_set: Arc<PrimitiveOperationSet>,
}

impl Default for KExpressions {
    fn default() -> Self {
        KExpressions {
            params: KExpressionParams::default(),
            operations_set: Arc::new(PrimitiveOperationSet::new(vec![], 0)),
        }
    }
}

impl KExpressions {
    pub fn single_root_primitives(operations_set: PrimitiveOperationSet, length: u32) -> KExpressions {
        KExpressions::new(operations_set, KExpressionParams::new(0, 0, length, 1, false))
    }

    pub fn single_root_adfs(operations_set: PrimitiveOperationSet,
                            root_length: u32,
                            sub_length: u32,
                            sub_count: u32) -> KExpressions {
        KExpressions::new(operations_set, KExpressionParams::new(sub_length, sub_count, root_length, 1, false))
    }

    pub fn new(operations_set: PrimitiveOperationSet,
               params: KExpressionParams,
               // reusable_sub_expr: Option<bool>
           ) -> KExpressions {

        KExpressions {
            params,
            operations_set: Arc::new(operations_set),
        }
    }

    pub fn new_k_expr(&self) -> KExpression {
        // TODO: variable args count (?)
        let sub_expr_args_count = Some(2);
        let mut ids = self.operations_set.operation_ids(sub_expr_args_count);
        let mut k_expr: Vec<u32> = vec![];

        for _ in 0..self.params.subs_count {
            let sub_expr_ops = ids.random_ids(self.params.sub_length);
            k_expr.extend(&sub_expr_ops);

            if self.params.reuse_sub_expr {
                ids.add_exprs(1);
            }
        }

        let mut ids = self.operations_set.operation_ids(None);
        ids.add_exprs(self.params.subs_count);

        for _ in 0..self.params.roots_count {
            let root_expr_ops = ids.random_ids(self.params.root_length);
            k_expr.extend(&root_expr_ops);
        }

        KExpression {
            value: k_expr,
            params: self.params,
            primitives_set: Arc::clone(&self.operations_set),
        }
    }
}
