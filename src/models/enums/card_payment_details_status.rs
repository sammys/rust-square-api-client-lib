//! Model for CardPaymentDetailsStatus enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Card payment state.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsStatus {
    Authorized,
    Captured,
    Voided,
    Failed,
}
