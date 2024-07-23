//! Model for CardPaymentDetailsCvvStatus enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Status code returned from the Card Verification Value (CVV) check.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsCvvStatus {
    CvvAccepted,
    CvvRejected,
    CvvNotChecked,
}
