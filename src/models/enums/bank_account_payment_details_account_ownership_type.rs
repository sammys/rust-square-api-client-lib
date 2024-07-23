//! Model for BankAccountPaymentDetailsAccountOwnershipType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// An ownership type of a bank account performing a transfer for payment.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountPaymentDetailsAccountOwnershipType {
    Individual,
    Company,
    Unknown,
}
