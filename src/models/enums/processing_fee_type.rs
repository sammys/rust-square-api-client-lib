//! Model for ProcessingFeeType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The type of fee assessed or adjusted.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessingFeeType {
    Initial,
    Adjustment,
}
