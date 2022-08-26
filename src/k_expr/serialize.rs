use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::utils::filesystem as fs;
use crate::operations::op_set::PrimitiveOperationSet;

use super::core::{KExpression, KExpressionParams};

#[derive(Clone, Serialize, Deserialize)]
pub struct KExpressionSer {
    value: Vec<u32>,
    params: KExpressionParams,
    primitives_set: PrimitiveOperationSet,
}

impl KExpressionSer {
    pub fn from_k_expr(k_expr: &KExpression) -> KExpressionSer {
        KExpressionSer {
            value: k_expr.value.clone(),
            params: k_expr.params,
            primitives_set: (*k_expr.primitives_set).clone()
        }
    }

    pub fn to_k_expr(self) -> KExpression {
        KExpression {
            value: self.value,
            params: self.params,
            primitives_set: Arc::new(self.primitives_set)
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        fs::serialize_to_file(filename, self)?;
        Ok(())
    }

    pub fn restore(filename: &str) -> std::io::Result<KExpression> {
        let k_expr_data: KExpressionSer = fs::deserialize_from_file(filename)?;
        Ok(k_expr_data.to_k_expr())
    }
}
