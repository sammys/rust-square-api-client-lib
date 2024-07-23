//! Model for AchDetailsAccountType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// A type of bank account performing a transfer for payment.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AchDetailsAccountType {
    Checking,
    Savings,
    Unknown,
}
