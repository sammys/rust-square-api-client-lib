//! Model for SubscriptionStatus enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Supported subscription statuses.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    /// The subscription is pending to start in the future.
    Pending,
    /// The subscription is active.
    Active,
    /// The subscription is canceled.
    Canceled,
    /// The subscription is deactivated.
    Deactivated,
    /// the subscription is paused.
    Paused,
}
