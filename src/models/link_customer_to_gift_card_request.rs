//! The request body for the Link Customer To Gift Card API

use crate::{Hydrate, Reconcile};
use serde::Serialize;

/// This is a model struct for LinkCustomerToGiftCardRequest type
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct LinkCustomerToGiftCardRequest {
    /// The ID of the customer to link to the gift card.
    ///
    /// Min Length: 1, Max Length: 191
    pub customer_id: String,
}
