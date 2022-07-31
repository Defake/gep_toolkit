# GEP Toolkit
Implementation of Gene Expression Programming in Rust.

### Usage

Add the GEP Toolkit dependency in your Cargo.toml file:

```toml
[dependencies]
gep_toolkit = "0.1.0"
```

Use `KExpression`s as your genetic population chromosomes, and use `k_expr.expression(RGEP)` to build an expression tree and compute it. 

```rust
fn main() {
    let operations: Vec<PrimitiveOperation> = vec![
        op::CONST_1.primitive(),
        op::CONST_NEG_1.primitive(),
        op::OPERATOR_PLUS.primitive(),
        op::OPERATOR_MULTIPLY.primitive(),
        op::OPERATOR_POW.primitive(),
    ];

    let set = PrimitiveOperationSet::new(operations, 2);
    let ctx = KExpressions::single_root_primitives(set, 15);

    let k_expr = ctx.new_k_expr();
    let root_expr = k_expr.expression(ExpressionTreeType::RGEP);

    let args = vec![1.0, 2.0];
    println!("{:?}", root_expr.compute_result(&args));
}
```

Note that the library is intended for expression trees generation and computing them. In order to run a simulation, you need to use a separate GA library. Check out examples.

### Examples

There's an [example](https://github.com/Defake/gep_toolkit/tree/master/examples/oxigen_math_expression) of running a GEP simulation on [oxigen](https://github.com/Martin1887/oxigen).

### What is supported?
* Currently only RGEP expression parsing is implemented. GEP and PGEP are TODO
* ADFs and SLEs are supported, but `KExpression.mutate()` doesn't support consider sub-expression positions, so you may want to implement your own mutate function.  

