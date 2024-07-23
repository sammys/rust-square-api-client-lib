//! Model struct for GiftCardActivityClearBalance type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents details about a `CLEAR_BALANCE` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct GiftCardActivityClearBalance {
    /// The reason the gift card balance was cleared.
    pub reason: String,
}
