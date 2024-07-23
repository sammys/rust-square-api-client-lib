//! Model for OrderServiceChargeType enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The type of the service charge.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeType {
    AutoGratuity,
    Custom,
}
