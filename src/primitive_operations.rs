use std::f64::consts;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub func: fn(f64) -> f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    pub func: fn(f64, f64) -> f64,
}


#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveOperation {
    Constant(Constant, &'static str),
    Argument(Argument),
    Modifier(Modifier, &'static str),
    Operator(Operator, &'static str),
}
impl fmt::Display for PrimitiveOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display = match self {
            PrimitiveOperation::Constant(c, display) => display.to_string(),
            PrimitiveOperation::Argument(arg) => format!("[{}]", arg.index),
            PrimitiveOperation::Modifier(func, display) => display.to_string(),
            PrimitiveOperation::Operator(op, display) => display.to_string(),
        };
        write!(f, "{}", display)
    }
}
impl Serialize for PrimitiveOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}


pub trait PrimitiveOperationConstructor {
    fn primitive(self) -> PrimitiveOperation;
}
// TODO: remove? only used by tests
impl PrimitiveOperationConstructor for Argument {
    fn primitive(self) -> PrimitiveOperation {
        PrimitiveOperation::Argument(self)
    }
}


pub const _CONST_1: Constant = Constant { value: 1_f64 };
pub const CONST_1: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_1, "1");

pub const _CONST_10: Constant = Constant { value: 10_f64 };
pub const CONST_10: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_10, "10");

pub const _CONST_100: Constant = Constant { value: 100_f64 };
pub const CONST_100: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_100, "100");

pub const _CONST_1000: Constant = Constant { value: 1000_f64 };
pub const CONST_1000: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_1000, "1000");

pub const _CONST_NEG_1: Constant = Constant { value: -1_f64 };
pub const CONST_NEG_1: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_NEG_1, "-1");

pub const _CONST_PI: Constant = Constant { value: consts::PI };
pub const CONST_PI: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_PI, "PI");

pub const _CONST_EPSILON: Constant = Constant { value: f64::EPSILON };
pub const CONST_EPSILON: PrimitiveOperation = PrimitiveOperation::Constant(_CONST_EPSILON, "EPS");


pub const _MODIFIER_SQUARE: Modifier = Modifier {
    func: |x| {
        x.powi(2)
    },
};
pub const MODIFIER_SQUARE: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_SQUARE, "^2");

pub const _MODIFIER_SQRT: Modifier = Modifier { func: f64::sqrt };
pub const MODIFIER_SQRT: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_SQRT, "Q");

pub const _MODIFIER_SIN: Modifier = Modifier { func: f64::sin };
pub const MODIFIER_SIN: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_SIN, "Sin");

pub const _MODIFIER_COS: Modifier = Modifier { func: f64::cos };
pub const MODIFIER_COS: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_COS, "Cos");

pub const _MODIFIER_TANH: Modifier = Modifier { func: f64::tanh };
pub const MODIFIER_TANH: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_TANH, "Tanh");

pub const _MODIFIER_SIGMOID: Modifier = Modifier {
    func: |x| {
        1_f64 / (1_f64 + consts::E.powf(-x))
    },
};
pub const MODIFIER_SIGMOID: PrimitiveOperation = PrimitiveOperation::Modifier(_MODIFIER_SIGMOID, "Sigm");

pub const _OPERATOR_PLUS: Operator = Operator {
    func: |a, b| {
        a + b
    },
};
pub const OPERATOR_PLUS: PrimitiveOperation = PrimitiveOperation::Operator(_OPERATOR_PLUS, "+");

pub const _OPERATOR_MINUS: Operator = Operator {
    func: |a, b| {
        a - b
    },
};
pub const OPERATOR_MINUS: PrimitiveOperation = PrimitiveOperation::Operator(_OPERATOR_MINUS, "-");

pub const _OPERATOR_MULTIPLY: Operator = Operator {
    func: |a, b| {
        a * b
    },
};
pub const OPERATOR_MULTIPLY: PrimitiveOperation = PrimitiveOperation::Operator(_OPERATOR_MULTIPLY, "*");

pub const _OPERATOR_DIVIDE: Operator = Operator {
    func: |a, b| {
        a / b
    },
};
pub const OPERATOR_DIVIDE: PrimitiveOperation = PrimitiveOperation::Operator(_OPERATOR_DIVIDE, "/");

pub const _OPERATOR_POW: Operator = Operator {
    func: |a, b| {
        a.powf(b)
    },
};
pub const OPERATOR_POW: PrimitiveOperation = PrimitiveOperation::Operator(_OPERATOR_POW, "^");

