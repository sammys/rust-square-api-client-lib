//! The request body for the Unlink Customer From Gift Card API

use crate::{Hydrate, Reconcile};
use serde::Serialize;

/// This is a model struct for UnlinkCustomerFromGiftCardRequest type
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UnlinkCustomerFromGiftCardRequest {
    /// The ID of the customer to unlink from the gift card.
    ///
    /// Min Length: 1, Max Length: 191
    pub customer_id: String,
}
