//! Model for PaymentDelayAction enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Actions that can be applied to [Payment]s when the `delay_duration` has elapsed.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentDelayAction {
    /// Cancel the payment.
    Cancel,
}
