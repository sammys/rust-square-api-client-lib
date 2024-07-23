//! Model for MeasurementUnitGeneric enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Reserved for API integrations that lack the ability to specify a real measurement unit.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitGeneric {
    /// The generic unit.
    Unit,
}
