//! Model for OrderFulfillmentPickupDetailsScheduleType enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The schedule type of the pickup fulfillment.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentPickupDetailsScheduleType {
    /// Indicates that the fulfillment will be picked up at a scheduled pickup time.
    Scheduled,
    /// Indicates that the fulfillment will be picked up as soon as possible and should be prepared
    /// immediately.
    Asap,
}
