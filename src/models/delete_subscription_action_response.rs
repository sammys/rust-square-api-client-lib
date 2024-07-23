//! Response struct for the Delete Subscription Action API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for DeleteSubscriptionActionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct DeleteSubscriptionActionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The subscription that has the specified action deleted.
    pub subscription: Subscription,
}
