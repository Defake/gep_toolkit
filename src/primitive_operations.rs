use std::f64::consts;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    C1,
    C2,
    C3,
    C10,
    C100,
    C1000,
    CNeg1,
    Pi,
    E,
    Epsilon,
}
impl Constant {
    pub fn value(&self) -> f64 {
        match self {
            Constant::C1 => 1_f64,
            Constant::C2 => 2_f64,
            Constant::C3 => 3_f64,
            Constant::C10 => 10_f64,
            Constant::C100 => 100_f64,
            Constant::C1000 => 1000_f64,
            Constant::CNeg1 => -1_f64,
            Constant::Pi =>  consts::PI,
            Constant::E =>  consts::E,
            Constant::Epsilon => f64::EPSILON,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Argument {
    Arg(u32)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Sqr,
    Pow3,
    Sqrt,
    Log2,
    Log10,
    Sin,
    Cos,
    Tanh,
    Sigmoid,
}
impl Modifier {
    pub fn compute(&self, x: f64) -> f64 {
        match self {
            Modifier::Sqr => x.powi(2),
            Modifier::Pow3 => x.powi(3),
            Modifier::Sqrt => x.sqrt(),
            Modifier::Log2 => x.log2(),
            Modifier::Log10 => x.log10(),
            Modifier::Sin => x.sin(),
            Modifier::Cos => x.cos(),
            Modifier::Tanh => x.tanh(),
            Modifier::Sigmoid => {
                1_f64 / (1_f64 + consts::E.powf(-x))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow,
    Root,
    Log,
}
impl Operator {
    pub fn compute(&self, x: f64, y: f64) -> f64 {
        match self {
            Operator::Plus => x + y,
            Operator::Minus => x - y,
            Operator::Multiply => x * y,
            Operator::Divide => x / y,
            Operator::Pow => x.powf(y),
            Operator::Root => x.powf(1_f64/y),
            Operator::Log => x.log(y),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveOperation {
    Constant(Constant),
    Argument(Argument),
    Modifier(Modifier),
    Operator(Operator),
}

impl fmt::Display for PrimitiveOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveOperation::Constant(c) => write!(f, "{:?}", c),
            PrimitiveOperation::Argument(a) => write!(f, "{:?}", a),
            PrimitiveOperation::Modifier(m) => write!(f, "{:?}", m),
            PrimitiveOperation::Operator(o) => write!(f, "{:?}", o),
        }
    }
}
