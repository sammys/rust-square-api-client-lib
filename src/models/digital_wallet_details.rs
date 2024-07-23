//! Model struct for DigitalWalletDetails type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::enums::CardPaymentDetailsStatus;

/// Additional details about `WALLET` type payments.
///
/// Contains only non-confidential information.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct DigitalWalletDetails {
    /// The status of the `WALLET` payment.
    pub status: Option<CardPaymentDetailsStatus>,
}
