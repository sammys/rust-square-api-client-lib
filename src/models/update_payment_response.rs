//! Model struct for UpdatePaymentResponse type.

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Payment};

/// This is a model struct for UpdatePaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct UpdatePaymentResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated payment.
    pub payment: Option<Payment>,
}
