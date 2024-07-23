//! Model struct for GiftCardActivityBlock type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents details about a `BLOCK` [gift card activity type](GiftCardActivityType).
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct GiftCardActivityBlock {
    /// The reason the gift card was blocked.
    pub reason: String,
}
