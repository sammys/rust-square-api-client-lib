//! Model struct for BusinessHours type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::BusinessHoursPeriod;

/// The hours of operation for a location.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct BusinessHours {
    /// The list of time periods during which the business is open. There can be at most 10 periods
    /// per day.
    pub periods: Option<Vec<BusinessHoursPeriod>>,
}
