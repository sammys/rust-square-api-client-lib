//! Model struct for AfterpayDetails type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Additional details about Afterpay payments.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct AfterpayDetails {
    /// Email address on the buyer's Afterpay account.
    pub email_address: Option<String>,
}
