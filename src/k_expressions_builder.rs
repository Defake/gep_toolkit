use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use std::option::Option;
use std::fmt;
use crate::operation_ids::OperationIds;
use super::primitive_operations as ops;
use super::operation_set::*;
use super::expressions as exp;
use super::k_expression::KExpression;


pub struct KExpressions {
    pub sub_expr_length: u32,
    pub sub_expr_count: u32,
    pub root_expr_length: u32,
    pub root_expr_count: u32,
    // TODO: make an option to ensure the tree is always full width
    // full_width: bool,
    pub operations_set: Arc<PrimitiveOperationSet>,

    pub reusable_sub_expr: bool,
}

impl Default for KExpressions {
    fn default() -> Self {
        KExpressions {
            sub_expr_length: 0,
            sub_expr_count: 0,
            root_expr_length: 0,
            root_expr_count: 0,
            operations_set: Arc::new(PrimitiveOperationSet::new(vec![], 0)),
            reusable_sub_expr: false
        }
    }
}

impl KExpressions {
    pub fn single_root_primitives(operations_set: PrimitiveOperationSet, length: u32) -> KExpressions {
        KExpressions::new(operations_set, 0, 0, length, 1)
    }

    pub fn single_root_adfs(operations_set: PrimitiveOperationSet,
                            root_length: u32,
                            sub_length: u32,
                            sub_count: u32) -> KExpressions {
        KExpressions::new(operations_set, sub_length, sub_count, root_length, 1)
    }

    pub fn new(operations_set: PrimitiveOperationSet,
               sub_expr_length: u32,
               sub_expr_count: u32,
               root_expr_length: u32,
               root_expr_count: u32,
               // reusable_sub_expr: Option<bool>
           ) -> KExpressions {

        KExpressions {
            operations_set: Arc::new(operations_set),
            sub_expr_length: sub_expr_length,
            sub_expr_count: sub_expr_count,
            root_expr_length: root_expr_length,
            root_expr_count: root_expr_count,
            reusable_sub_expr: false,
        }
    }

    pub fn new_k_expr(&self) -> KExpression {
        let mut ids = self.operations_set.operation_ids();
        let mut k_expr: Vec<u32> = vec![];

        for _ in 0..self.sub_expr_count {
            let sub_expr_ops = ids.random_ids(self.sub_expr_length);
            k_expr.extend(&sub_expr_ops);

            if self.reusable_sub_expr {
                ids.add_exprs(1);
            }
        }

        if !self.reusable_sub_expr {
            ids.add_exprs(self.sub_expr_count);
        }

        for _ in 0..self.root_expr_count {
            let root_expr_ops = ids.random_ids(self.root_expr_length);
            k_expr.extend(&root_expr_ops);
        }

        KExpression {
            value: k_expr,
            primitives_set: Arc::clone(&self.operations_set),
            sub_length: self.sub_expr_length,
            subs_count: self.sub_expr_count,
            root_length: self.root_expr_length,
            roots_count: self.root_expr_count,
        }
    }
}


