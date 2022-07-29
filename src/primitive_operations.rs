use std::f64::consts;

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


#[derive(Debug, Clone, PartialEq)] //, Sync
pub enum PrimitiveOperation {
    Constant(Constant),
    Argument(Argument),
    Modifier(Modifier),
    Operator(Operator),
}

pub trait PrimitiveOperationConstructor {
    fn primitive(self) -> PrimitiveOperation;
}

impl PrimitiveOperationConstructor for Constant {
    fn primitive(self) -> PrimitiveOperation {
        PrimitiveOperation::Constant(self)
    }
}

impl PrimitiveOperationConstructor for Argument {
    fn primitive(self) -> PrimitiveOperation {
        PrimitiveOperation::Argument(self)
    }
}

impl PrimitiveOperationConstructor for Modifier {
    fn primitive(self) -> PrimitiveOperation {
        PrimitiveOperation::Modifier(self)
    }
}

impl PrimitiveOperationConstructor for Operator {
    fn primitive(self) -> PrimitiveOperation {
        PrimitiveOperation::Operator(self)
    }
}


pub const CONST_1: Constant = Constant { value: 1_f64 };
pub const CONST_10: Constant = Constant { value: 10_f64 };
pub const CONST_100: Constant = Constant { value: 100_f64 };
pub const CONST_1000: Constant = Constant { value: 1000_f64 };
pub const CONST_NEG_1: Constant = Constant { value: -1_f64 };


pub const MODIFIER_SQUARE: Modifier = Modifier {
    func: |x| {
        x.powi(2)
    }
};
pub const MODIFIER_SQRT: Modifier = Modifier {
    func: f64::sqrt
};
pub const MODIFIER_SIN: Modifier = Modifier {
    func: f64::sin
};
pub const MODIFIER_COS: Modifier = Modifier {
    func: f64::cos
};
pub const MODIFIER_TANH: Modifier = Modifier {
    func: f64::tanh
};
pub const MODIFIER_SIGMOID: Modifier = Modifier {
    func: |x| {
        1_f64 / (1_f64 + consts::E.powf(-x))
    }
};


pub const OPERATOR_PLUS: Operator = Operator {
    func: |a, b| {
        a + b
    }
};
pub const OPERATOR_MINUS: Operator = Operator {
    func: |a, b| {
        a - b
    }
};
pub const OPERATOR_MULTIPLY: Operator = Operator {
    func: |a, b| {
        a * b
    }
};
pub const OPERATOR_DIVIDE: Operator = Operator {
    func: |a, b| {
        a / b
    }
};
pub const OPERATOR_POW: Operator = Operator {
    func: |a, b| {
        a.powf(b)
    }
};

