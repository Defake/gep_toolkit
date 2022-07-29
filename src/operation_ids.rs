use rand::Rng;

pub struct OperationIds {
    operations_count: u32,
    expr_count: u32,
}

impl OperationIds {
    pub fn new(operations_count: u32) -> OperationIds {
        OperationIds {
            operations_count,
            expr_count: 0,
        }
    }

    pub fn random_id(&self) -> u32 {
        let available_items = self.operations_count + self.expr_count;

        if available_items == 0 {
            panic!("No available operations to pick from");
        }

        let mut rng = rand::thread_rng();

        let result: u32 = rng.gen_range(0, available_items).try_into().unwrap();
        result
    }

    pub fn random_ids(&self, length: u32) -> Vec<u32> {
        let mut value = vec![];
        // self.value.push(self.genes_set.random_non_terminal(self.sub_expr_amount));
        for _ in 0..length {
            value.push(self.random_id())
        }

        value
    }

    pub fn add_exprs(&mut self, add_count: u32) {
        self.expr_count += add_count;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_operation_ids_panics() {
        let op_ids = OperationIds::new(0);
        op_ids.random_id();
    }

    #[test]
    fn test_random_ids_are_within_range() {
        let op_ids = OperationIds::new(10);
        let ids = op_ids.random_ids(10000);
        assert_eq!(ids.len(), 10000);

        let mut options: Vec<bool> = vec![false; 10];
        for id in ids {
            options[id as usize] = true;
        }

        for i in 0..10_usize {
            assert_eq!(options[i], true)
        }
    }

    #[test]
    fn test_random_expr_ids_are_within_range() {
        let mut op_ids = OperationIds::new(10);
        op_ids.add_exprs(10);

        let ids = op_ids.random_ids(10000);

        let mut options: Vec<bool> = vec![false; 20];
        for id in ids {
            options[id as usize] = true;
        }

        for i in 0..20_usize {
            assert_eq!(options[i], true)
        }
    }
}
