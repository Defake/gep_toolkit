# GEP Toolkit
Implementation of Gene Expression Programming in Rust. Supports SL-GEP (Self-Learning Gene Expression Programming), check out references at the bottom of this README.

### Usage

Add the GEP Toolkit dependency in your Cargo.toml file:

```toml
[dependencies]
gep_toolkit = "0.2.1"
```

Use `KExpression`s as your genetic population chromosomes, and use `k_expr.expression(ExpressionTreeType::RGEP)` to build an expression tree and compute it (GEP and PGEP are not supported yet). 

```rust
use gep_toolkit::operations::primitives::*;
use gep_toolkit::operations::op_set::PrimitiveOperationSet;
use gep_toolkit::k_expr::builder::KExpressions;
use gep_toolkit::k_expr::core::{ExpressionTreeType, KExpressionParams};
use gep_toolkit::k_expr::serialize::KExpressionSer;

fn main() {
    let operations: Vec<PrimitiveOperation> = vec![
        PrimitiveOperation::Constant(Constant::CNeg1),
        PrimitiveOperation::Constant(Constant::C1),
        PrimitiveOperation::Constant(Constant::C2),
        PrimitiveOperation::Constant(Constant::C3),
        PrimitiveOperation::Constant(Constant::C10),
        PrimitiveOperation::Constant(Constant::C100),
        PrimitiveOperation::Constant(Constant::C1000),
        PrimitiveOperation::Modifier(Modifier::Sqr),
        PrimitiveOperation::Modifier(Modifier::Pow3),
        PrimitiveOperation::Modifier(Modifier::Sqrt),
        PrimitiveOperation::Modifier(Modifier::Log2),
        PrimitiveOperation::Modifier(Modifier::Log10),
        PrimitiveOperation::Modifier(Modifier::Sin),
        PrimitiveOperation::Modifier(Modifier::Cos),
        PrimitiveOperation::Modifier(Modifier::Tanh),
        PrimitiveOperation::Modifier(Modifier::Sigmoid),
        PrimitiveOperation::Operator(Operator::Plus),
        PrimitiveOperation::Operator(Operator::Minus),
        PrimitiveOperation::Operator(Operator::Multiply),
        PrimitiveOperation::Operator(Operator::Divide),
        PrimitiveOperation::Operator(Operator::Pow),
        PrimitiveOperation::Operator(Operator::Root),
        PrimitiveOperation::Operator(Operator::Log),
    ];
    
    let args_count = 2;
    let set = PrimitiveOperationSet::new(operations, args_count);
    let params = KExpressionParams {
        root_length: 5,
        sub_length: 10,
        subs_count: 3,
        reuse_sub_expr: true,
        ..KExpressionParams::default()
    };
    let ctx = KExpressions::new(set, params);

    let k_expr = ctx.new_k_expr();
    let root_expr = k_expr.expression(ExpressionTreeType::RGEP);

    let args = vec![1.0, 2.0];
    println!("{:?}", root_expr.compute_result(&args));
}
```

Note that the library is intended for expression trees generation and computing them. In order to run a simulation, you will need to use a separate GA library. Check out examples.

### Examples
* [Example 1](https://github.com/Defake/gep_toolkit/tree/master/examples/oxigen_math_expression/src/main.rs) – running a GEP simulation on [oxigen](https://github.com/Martin1887/oxigen)
* [Example 2](https://github.com/Defake/gep_toolkit/tree/master/examples/saving_loading/src/main.rs) – save/load operation set and K-Expressions

### TODO
- [x] Saving/loading operation set and expressions
- [x] Support `KExpression.mutate()` with regard to ADFs and SLEs positions
- [ ] More concise K-Expression display format
- [ ] GEP and PGEP expressions parsing (only RGEP is supported currently)
- [ ] Support pure ADFs without arguments
- [ ] Support restricting usage of primitive operations in main expression to support only-ADFs approach


### References
* [Gene Expression Programming, 2001 – Candida Ferreira](https://arxiv.org/abs/cs/0102027)
* [Prefix Gene Expression Programming, 2005 – Xin Li, et al.](https://www.cs.uic.edu/~xli1/papers/PGEP_GECCOLateBreaking05_XLi.pdf)
* [Automatically Defined Functions in Gene Expression Programming, 2006 - Candida Ferreira](https://www.semanticscholar.org/paper/Automatically-Defined-Functions-in-Gene-Expression-Ferreira/2f3ccc2ccc2992b07f7fe09948eabb54fbe6e61b)
* [Robust Gene Expression Programming, 2011 – Noah Ryan, et al.](https://www.sciencedirect.com/science/article/pii/S1877050911004972)
* [Self-Learning Gene Expression Programming, 2015 – Jinghui Zhong, at al.](https://www.researchgate.net/publication/276136922_Self-Learning_Gene_Expression_Programming)
