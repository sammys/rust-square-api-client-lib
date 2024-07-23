//! Request body struct for the Update Subscription API

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::Subscription;

/// This is a model struct for UpdateSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UpdateSubscriptionRequest {
    /// The subscription object containing the current version, and fields to update. Unset fields
    /// will be left at their current server values, and JSON `null` values will be treated as a
    /// request to clear the relevant data.
    pub subscription: Option<Subscription>,
}
