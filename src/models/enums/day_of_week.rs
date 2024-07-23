//! Model for DayOfWeek enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Indicates the specific day of the week.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    /// Sunday
    Sun,
    /// Monday
    Mon,
    /// Tuesday
    Tue,
    /// Wednesday
    Wed,
    /// Thursday
    Thu,
    /// Friday
    Fri,
    /// Saturday
    Sat,
}
