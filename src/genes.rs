use rand::Rng;
use std::rc::Rc;
use std::collections::HashMap;
use crate::expressions::StackOperation;
use super::utils::collections::hashmap;
use super::expressions as exp;
use super::operations as ops;

#[derive(Clone)]
pub struct GenesSet {
    operations_set: Vec<Rc<dyn exp::StackOperation>>,
    adfs_set: Vec<Rc<dyn exp::StackOperation>>,
    operations_count: u32,
    // TODO: don't control how many adfs should be in the set from the GenesSet struct
    adfs_count: u8,
    args_count: u32,
}

impl GenesSet {
    pub fn new(mut operations: Vec<Rc<dyn exp::StackOperation>>, args_amount: u32) -> GenesSet {
        for op in &operations {
            if !op.is_pure_operation() {
                panic!("Operation {:?} can not be passed as a custom operation for Genes Set", op)
            }
        }

        for arg_i in 0..args_amount {
            operations.push(Rc::new(ops::Argument { index: arg_i }));
        }

        GenesSet {
            operations_count: operations.len() as u32,
            operations_set: operations,
            adfs_set: vec![],
            adfs_count: 0,
            args_count: args_amount,
        }
    }
}

pub trait GenesProducer {
    fn random_gene_id(&self) -> u32;
    fn dictionary(self: Box<Self>) -> Box<dyn GenesDictionary>;
    fn with_adfs(&self, available_adfs: u8) -> Box<dyn GenesProducer>;
}

impl GenesProducer for GenesSet {
    fn random_gene_id(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let available_items = self.operations_set.len() + self.adfs_count as usize;

        if available_items == 0 {
            panic!("No available genes to pick from");
        }

        let result:u32 = rng.gen_range(0, available_items).try_into().unwrap();
        result
    }

    fn dictionary(self: Box<Self>) -> Box<dyn GenesDictionary> {
        Box::new(*self)
    }

    fn with_adfs(&self, available_adfs: u8) -> Box<dyn GenesProducer> {
        Box::new(GenesSet {
            // TODO: reference
            operations_set: self.operations_set.clone(),
            operations_count: self.operations_count,
            args_count: self.args_count,
            adfs_set: vec![],
            adfs_count: available_adfs,
        })
    }
}

pub trait GenesDictionary {
    // TODO: take ownership of self
    fn with_adfs(&self, exprs: Vec<Rc<dyn exp::StackOperation>>) -> Box<dyn GenesDictionary/*<'b> + 'b*/>;
    fn gene_by_id(&self, id: u32) -> Rc<dyn exp::StackOperation>;
    fn args_amount(&self) -> u32;
}

impl GenesDictionary for GenesSet {
    fn with_adfs(&self, exprs: Vec<Rc<dyn exp::StackOperation>>) -> Box<dyn GenesDictionary/*<'b> + 'b*/> {
        if exprs.len() != self.adfs_count as usize {
            panic!("This GenesDictionary expected {} adf expressions, got: {}.",
                   self.adfs_count, exprs.len())
        }

        Box::new(
            GenesSet {
                // TODO: reference (Rc?)
                operations_set: self.operations_set.clone(),
                adfs_set: exprs,
                operations_count: self.operations_count,
                adfs_count: self.adfs_count,
                args_count: self.args_count,
            }
        )
    }

    fn gene_by_id(&self, id: u32) -> Rc<dyn exp::StackOperation> {
        // id is an index in GenesProducer's default implementation, so we can just use vector indexing
        if id < self.operations_count {
            return Rc::clone(&self.operations_set[id as usize]);
        }

        if id >= self.operations_count + self.adfs_count as u32 {
            panic!("ID {} is not available in GenesDictionary with {} operations and {} adfs",
                   id, self.operations_count, self.adfs_count);
        }

        let adf_id = id - self.operations_count;

        if adf_id >= self.adfs_set.len() as u32 {
            panic!("ADF ID {} is not available in GenesDictionary with {} stored ADFs and {} claimed ADFs",
                   adf_id, self.adfs_set.len(), self.adfs_count);
        }

        Rc::clone(&self.adfs_set[adf_id as usize])
    }

    fn args_amount(&self) -> u32 {
        self.args_count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_when_passed_arguments_to_constructor() {
        GenesSet::new(vec![Rc::new(ops::Argument { index: 0 })], 1);
    }

    #[test]
    fn genes_producing_test() {
        let set = GenesSet {
            operations_set: vec![Rc::new(ops::Argument { index: 0 }),
                                 Rc::new(ops::CONST_1),
                                 Rc::new(ops::MODIFIER_SQUARE),
                                 Rc::new(ops::OPERATOR_PLUS), ],
            adfs_set: vec![],
            operations_count: 4,
            adfs_count: 1,
            args_count: 1
        };


        let initial_set = set.clone();
        let gp: Box<dyn GenesProducer> = Box::new(set);

        // TODO: come on
        let id = gp.random_gene_id();
        assert_eq!(id < 4, true);

        let dict_0 = gp.dictionary();
        assert!(Rc::ptr_eq(&dict_0.gene_by_id(0), &initial_set.operations_set[0]));
        assert!(Rc::ptr_eq(&dict_0.gene_by_id(1), &initial_set.operations_set[1]));
        assert!(Rc::ptr_eq(&dict_0.gene_by_id(2), &initial_set.operations_set[2]));
        assert!(Rc::ptr_eq(&dict_0.gene_by_id(3), &initial_set.operations_set[3]));

        // (b^2) + a
        let sub_expr_operations = vec![
            Rc::clone(&initial_set.operations_set[0]),
            Rc::clone(&initial_set.operations_set[2]),
            Rc::clone(&initial_set.operations_set[1]),
            Rc::clone(&initial_set.operations_set[3]),
        ];

        let sub_expr: Rc<dyn exp::StackOperation> = Rc::new(
            exp::Expression {
                // (b^2) + a
                operations: sub_expr_operations.clone()
            });

        let dict_1 = dict_0.with_adfs(vec![Rc::clone(&sub_expr)]);
        assert!(Rc::ptr_eq(&dict_1.gene_by_id(0), &initial_set.operations_set[0]));
        assert!(Rc::ptr_eq(&dict_1.gene_by_id(1), &initial_set.operations_set[1]));
        assert!(Rc::ptr_eq(&dict_1.gene_by_id(2), &initial_set.operations_set[2]));
        assert!(Rc::ptr_eq(&dict_1.gene_by_id(3), &initial_set.operations_set[3]));

        assert!(Rc::ptr_eq(&dict_1.gene_by_id(4), &sub_expr));
    }
}
