//! Model for TaxCalculationPhase enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// When to calculate the taxes due on a cart.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxCalculationPhase {
    /// The fee is calculated based on the payment's subtotal.
    TaxSubtotalPhase,
    /// The fee is calculated based on the payment's total.
    TaxTotalPhase,
}
