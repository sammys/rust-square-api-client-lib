//! Model struct for GiftCardActivityUnblock type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents details about a `UNBLOCK` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct GiftCardActivityUnblock {
    /// The reason the gift card was unblocked.
    pub reason: String,
}
