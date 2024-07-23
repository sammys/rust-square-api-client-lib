//! Model for BankAccountPaymentDetailsTransferType type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// A type of bank transfer
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountPaymentDetailsTransferType {
    Ach,
    Unknown,
}
