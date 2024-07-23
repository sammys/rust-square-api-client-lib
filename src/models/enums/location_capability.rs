//! Model for LocationCapability enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The capabilities a location might have.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationCapability {
    /// The capability to process credit card transactions with Square.
    CreditCardProcessing,
    /// The capability to receive automatic transfers from Square.
    AutomaticTransfers,
}
