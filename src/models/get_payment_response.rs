//! Model struct for GetPaymentResponse type.

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Payment};

/// This is a model struct for GetPaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct GetPaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested [Payment]
    pub payment: Option<Payment>,
}
