use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use super::operations as ops;
use super::genes::*;
use super::expressions as exp;

pub struct KExpressionParams {
    root_expressions_amount: u8,
    root_expression_length: u32,
    sub_expressions_amount: u8,
    sub_expression_length: u32,
    // TODO: make an option to ensure the tree is always full width
    // full_width: bool,
    genes_producer: Rc<dyn GenesProducer>,
}

impl<'a> KExpressionParams {
    pub fn new(genes_producer: Rc<dyn GenesProducer>,
           sub_expressions_amount: u8,
           sub_expression_length: u32,
           root_expressions_amount: u8,
           root_expression_length: u32,
           ) -> KExpressionParams {
        KExpressionParams {
            sub_expressions_amount,
            sub_expression_length,
            root_expressions_amount,
            root_expression_length,
            genes_producer
        }
    }
}

pub enum ExpressionTreeType {
    GEP,
    PGEP,
    RGEP,
}

pub struct KExpression {
    pub value: Vec<u32>,
    pub adf_length: u32,
    pub adfs_amount: u8,
    pub root_expr_length: u32,
    pub root_expr_amount: u8,

    genes: Box<dyn GenesDictionary>,
}

impl KExpression {
    fn expression_from_part<'b>(&self, expr_range: std::ops::Range<usize>, genes: &(dyn GenesDictionary + 'b)) -> Expression {
        let k_expr_part = &self.value[expr_range];
        let mut expr_operations: Vec<Rc<dyn exp::StackOperation>> = Vec::new();
        for gene_id in k_expr_part {
            let gene = genes.gene_by_id(*gene_id);
            expr_operations.push(gene);
        }

        Expression { operations: expr_operations }
    }

    fn expression_tree(&self, expr_type: ExpressionTreeType) -> exp::RootExpression {
        match expr_type {
            ExpressionTreeType::GEP => {
                todo!()
            }

            ExpressionTreeType::PGEP => {
                todo!()
            }

            ExpressionTreeType::RGEP => {
                let mut cursor = 0_usize;
                let mut sub_expressions: Vec<Rc<dyn exp::StackOperation>> = Vec::new();

                // TODO: using sub expressions in sub expressions
                // let mut current_genes = self.genes.with_adfs(sub_expressions.clone());

                for _ in 0..self.adfs_amount {
                    let expr = self.expression_from_part(cursor..cursor + self.adf_length as usize, &*self.genes);
                    sub_expressions.push(Rc::new(expr));

                    cursor += self.adf_length as usize;

                    // TODO: using sub expressions in sub expressions
                    // current_genes = current_genes.with_adfs(sub_expressions.clone());
                }

                let genes_with_adfs = self.genes.with_adfs(sub_expressions);

                let mut root_exprs: Vec<Expression> = Vec::new();
                for _ in 0..self.root_expr_amount {
                    let expr = self.expression_from_part(cursor..cursor + self.root_expr_length as usize, &*genes_with_adfs);
                    root_exprs.push(expr);

                    cursor += self.root_expr_length as usize;
                }

                exp::RootExpression {
                    exprs: root_exprs,
                    args_amount: self.genes.args_amount(),
                }
            }
        }
    }
}


// TODO: why trait? just use the ctx
trait KExpressions {
    fn new_k_expr(&self) -> KExpression;
}

impl KExpressions for KExpressionParams {
    fn new_k_expr(&self) -> KExpression {
        let mut builder = KExpressionBuilder::new(self);

        for _ in 0..self.sub_expressions_amount {
            builder.add_sub_expression();
        }

        builder.build()
    }
}

use k_expr_builder::KExpressionBuilder;
use crate::expressions::{Expression, StackOperation};

mod k_expr_builder {
    use super::*;

    pub struct KExpressionBuilder {
        value: Vec<u32>,
        sub_expr_length: u32,
        sub_expr_amount: u8,
        root_expr_length: u32,
        root_expr_amount: u8,
        genes_producer: Rc<dyn GenesProducer>,
    }

    impl KExpressionBuilder {
        pub fn new(ctx: &KExpressionParams) -> KExpressionBuilder {
            KExpressionBuilder {
                value: vec![],
                sub_expr_amount: ctx.sub_expressions_amount,
                sub_expr_length: ctx.sub_expression_length,
                root_expr_length: ctx.root_expression_length,
                root_expr_amount: ctx.root_expressions_amount,
                genes_producer: Rc::clone(&ctx.genes_producer),
            }
        }

        pub fn add_sub_expression(&mut self) {
            // TODO: using sub expressions in sub expressions
            self.add_inner_expression(self.sub_expr_length, &*Rc::clone(&self.genes_producer));
        }

        fn add_inner_expression(&mut self, length: u32, genes_producer: &dyn GenesProducer) {
            // self.value.push(self.genes_set.random_non_terminal(self.sub_expr_amount));
            for _ in 0..length {
                self.value.push(genes_producer.random_gene_id())
            }

            // TODO: using sub expressions in sub expressions
            // self.sub_expr_amount += 1;
        }

        pub fn build(mut self) -> KExpression {
            let producer_with_adfs = self.genes_producer.with_adfs(self.sub_expr_amount);
            self.add_inner_expression(self.root_expr_length, &*producer_with_adfs);

            KExpression {
                value: self.value,
                genes: producer_with_adfs.dictionary(),
                adf_length: self.sub_expr_length,
                adfs_amount: self.sub_expr_amount,
                // adfs_amount: self.sub_expr_amount - 1,
                root_expr_length: self.root_expr_length,
                root_expr_amount: self.root_expr_amount,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::k_expressions::*;

    #[test]
    fn k_expr_contains_generated_genes() {
        let op0: Rc<dyn StackOperation> = Rc::new(ops::CONST_1);
        let op1: Rc<dyn StackOperation> = Rc::new(ops::MODIFIER_SQUARE);
        let op2: Rc<dyn StackOperation> = Rc::new(ops::OPERATOR_PLUS);

        let set = GenesSet::new(
            vec![
                Rc::clone(&op0),
                Rc::clone(&op1),
                Rc::clone(&op2),
            ],
            2,
        );

        let gp: Rc<dyn GenesProducer> = Rc::new(set);

        let ctx = KExpressionParams {
            genes_producer: gp,
            sub_expressions_amount: 0,
            sub_expression_length: 0,
            root_expressions_amount: 1,
            root_expression_length: 1000,
        };

        let k_expr = ctx.new_k_expr();
        assert_eq!(k_expr.value.len(), 1000);

        for gene_id in k_expr.value {
            let gene = k_expr.genes.gene_by_id(gene_id);
            match gene_id {
                0 => assert!(Rc::ptr_eq(&gene, &op0)),
                1 => assert!(Rc::ptr_eq(&gene, &op1)),
                2 => assert!(Rc::ptr_eq(&gene, &op2)),
                3 => assert!(!gene.is_pure_operation()),
                4 => assert!(!gene.is_pure_operation()),
                x => panic!("Unexpected gene_id: {}", x)
            }
        }
    }

    #[test]
    fn k_expr_builds_correct_expr_tree() {
        let op0: Rc<dyn StackOperation> = Rc::new(ops::CONST_1);
        let op1: Rc<dyn StackOperation> = Rc::new(ops::MODIFIER_SQUARE);
        let op2: Rc<dyn StackOperation> = Rc::new(ops::OPERATOR_PLUS);

        let set = GenesSet::new(
            vec![
                Rc::clone(&op0),
                Rc::clone(&op1),
                Rc::clone(&op2),
            ],
            2,
        );

        let gp: Rc<dyn GenesProducer> = Rc::new(set);

        let ctx = KExpressionParams {
            genes_producer: gp,
            sub_expressions_amount: 5,
            sub_expression_length: 100,
            root_expressions_amount: 1,
            root_expression_length: 1000,
        };

        let k_expr = ctx.new_k_expr();
        let root_expr = k_expr.expression_tree(ExpressionTreeType::RGEP);

        let args = vec![1f64];
        assert_eq!(root_expr.args_amount, 2);
        assert_eq!(root_expr.exprs.len(), 1);
        for expr in root_expr.exprs {
            assert_eq!(expr.operations.len(), 1000);
            for gene in expr.operations {
                let eq0 = Rc::ptr_eq(&gene, &op0);
                let eq1 = Rc::ptr_eq(&gene, &op1);
                let eq2 = Rc::ptr_eq(&gene, &op2);

                assert!(eq0 || eq1 || eq2 || !gene.is_pure_operation());
            }
        }
    }
}


