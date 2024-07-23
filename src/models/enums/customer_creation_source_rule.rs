//! Model for CustomerCreationSourceRule enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The rule to include or exclude a certain set of values.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerCreationSourceRule {
    /// Indicates that the values should be included based on the rule.
    Include,
    /// Indicates that the values should be excluded based on the rule.
    Exclude,
}
