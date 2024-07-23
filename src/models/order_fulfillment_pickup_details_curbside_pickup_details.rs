//! Model struct for OrderFulfillmentPickupDetailsCurbsidePickupDetails type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::DateTime;

/// Specific details for curbside pickup.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct OrderFulfillmentPickupDetailsCurbsidePickupDetails {
    /// Specific details for curbside pickup, such as parking number and vehicle model.
    pub curbside_details: Option<String>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates)
    /// indicating when the buyer arrived and is waiting for pickup.
    pub buyer_arrived_at: Option<DateTime>,
}
