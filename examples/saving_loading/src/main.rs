use gep_toolkit::operations::primitives::*;
use gep_toolkit::operations::op_set::PrimitiveOperationSet;
use gep_toolkit::k_expr::builder::KExpressions;
use gep_toolkit::k_expr::core::{ExpressionTreeType};
use gep_toolkit::k_expr::serialize::KExpressionSer;

fn main() {
    let operations: Vec<PrimitiveOperation> = vec![
        PrimitiveOperation::Constant(Constant::C100),
        PrimitiveOperation::Constant(Constant::C1),
        PrimitiveOperation::Constant(Constant::CNeg1),
        PrimitiveOperation::Operator(Operator::Plus),
        PrimitiveOperation::Operator(Operator::Multiply),
        PrimitiveOperation::Operator(Operator::Pow),
    ];

    let op_set = PrimitiveOperationSet::new(operations, 2);
    println!("Original operations set: {:?}", &op_set);

    // Save operation set
    op_set.save("ops.b").unwrap();

    // Load operation set (for example, to continue genetic simulation)
    let op_set = op_set.restore("ops.b").unwrap();
    println!("Restored operations set: {:?}", &op_set);

    let ctx = KExpressions::single_root_primitives(op_set, 10);
    let k_expr = ctx.new_k_expr();
    println!("Original k-expression: {}", k_expr);

    // Save K-Expression
    let k_data = KExpressionSer::from_k_expr(&k_expr);
    k_data.save("k_expr.b").unwrap();

    // Load K-Expression
    let k_expr = KExpressionSer::restore("k_expr.b").unwrap();
    println!("Restored k-expression: {}", k_expr);

    let expression = k_expr.expression(ExpressionTreeType::RGEP);
    let vec1 = expression.compute_result(&vec![1_f64, 2_f64]);
    println!("Calculation result: {:?}", vec1);
}
