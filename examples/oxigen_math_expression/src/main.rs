use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::fs::File;
use std::hash::{Hash, Hasher};

use gep_toolkit::k_expr::core::*;
use gep_toolkit::k_expr::builder::*;
use gep_toolkit::operations::op_set::*;
use gep_toolkit::operations::primitives::*;
use gep_toolkit::operations::primitives::{PrimitiveOperation};
use oxigen::prelude::*;
use rand::prelude::*;

#[derive(Clone)]
struct Chromosome(KExpression);

impl Display for Chromosome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        (&self.0 as &dyn Display).fmt(f)
    }
}

fn problem(x: f64, y: f64) -> f64 {
    (x * x) + (y * y) - 1.0
}


fn problem_set() -> Vec<Vec<f64>> {
    vec![
        vec![1.0, 2.0],
        vec![2.0, 3.0],
        vec![99.0, -1.0],
        vec![-50.0, -1.5],
        vec![-500.0, 1.5],
        vec![-0.01, 25.0],
        vec![10000.0, 10000.0],
        vec![-90000.0, -90000.0],
    ]
}

const FITNESS_MAX: f64 = 1_000_000_000_000.0;

impl Genotype<u32> for Chromosome {
    type ProblemSize = KExpressions;
    type GenotypeHash = u64;

    fn iter(&self) -> std::slice::Iter<u32> {
        self.0.value.iter()
    }

    fn into_iter(self) -> std::vec::IntoIter<u32> {
        self.0.value.into_iter()
    }

    fn from_iter<I: Iterator<Item=u32>>(&mut self, genes: I) {
        self.0.value = genes.collect();
    }

    fn generate(size: &Self::ProblemSize) -> Self {
        Chromosome(size.new_k_expr())
    }

    fn fitness(&self) -> f64 {
        let mut error_rate = 0.0;

        let expr = self.0.expression(ExpressionTreeType::RGEP);
        for args in problem_set() {
            let expected = problem(args[0], args[1]);
            let actual = expr.compute_result(&args)[0];
            let diff = (expected - actual).abs();
            error_rate += diff;

            if error_rate.is_infinite() || error_rate.is_nan() || error_rate > FITNESS_MAX {
                error_rate = FITNESS_MAX;
                break
            }
        }

        1_f64 / (error_rate + 1_f64)
    }

    fn mutate(&mut self, _rgen: &mut SmallRng, index: usize) {
        self.0.mutate(index);
    }

    fn is_solution(&self, fitness: f64) -> bool {
        fitness > 0.99999
    }

    fn hash(&self) -> Self::GenotypeHash {
        let mut hasher = DefaultHasher::new();
        self.0.value.hash(&mut hasher);
        hasher.finish()
    }
}

pub fn main() {
    let operations: Vec<PrimitiveOperation> = vec![
        PrimitiveOperation::Constant(Constant::C1),
        PrimitiveOperation::Constant(Constant::C100),
        PrimitiveOperation::Constant(Constant::CNeg1),
        PrimitiveOperation::Operator(Operator::Plus),
        PrimitiveOperation::Operator(Operator::Multiply),
        PrimitiveOperation::Operator(Operator::Pow),
    ];

    let set = PrimitiveOperationSet::new(operations, 2);
    let params = KExpressionParams {
        roots_count: 1,
        root_length: 5,
        sub_length: 10,
        subs_count: 3,
        reuse_sub_expr: false
    };
    let ctx = KExpressions::new(set, params);

    let population_size = 1000;
    let progress_log = File::create("progress.csv").expect("Error creating progress log file");
    let population_log =
        File::create("population.txt").expect("Error creating population log file");

    let (solutions, generation, _progress, _population) = GeneticExecution::<u32, Chromosome>::new()
        .population_size(population_size)
        .genotype_size(ctx)
        .mutation_rate(Box::new(MutationRates::Constant(0.1_f64)))
        .selection_rate(Box::new(SelectionRates::Constant(10)))
        .select_function(Box::new(SelectionFunctions::Tournaments(NTournaments(population_size))))
        .progress_log(100, progress_log)
        .population_log(100, population_log)
        .run();

    println!("Finished in the generation {}", generation);
    for sol in &solutions {
        println!("{} - {}", sol, sol.fitness());
    }
}


