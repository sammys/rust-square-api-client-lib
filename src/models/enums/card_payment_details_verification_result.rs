//! Model for CardPaymentDetailsVerificationResult enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Results of cardholder verification.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsVerificationResult {
    Success,
    Failure,
    Unknown,
}
