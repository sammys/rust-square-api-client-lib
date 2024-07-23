//! Response struct for the Create Subscription API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Subscription};

/// This is a model struct for CreateSubscriptionResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreateSubscriptionResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created subscription.
    ///
    /// For more information, see [Subscription
    /// object](https://developer.squareup.com/docs/subscriptions-api/overview#subscription-object).
    pub subscription: Subscription,
}
