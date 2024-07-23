//! Model for RiskEvaluationRiskLevel enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The risk level associated with a payment.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskEvaluationRiskLevel {
    /// Indicates Square is still evaluating the payment.
    Pending,
    /// Indicates payment risk is within the normal range.
    Normal,
    /// Indicates elevated risk level associated with the payment.
    Moderate,
    /// Indicates significantly elevated risk level with the payment.
    High,
}
