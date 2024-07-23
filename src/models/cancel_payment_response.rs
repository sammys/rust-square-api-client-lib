//! Model struct for CancelPaymentResponse type.

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Payment};

/// This is a model struct for CancelPaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CancelPaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The successfully canceled [Payment] object.
    pub payment: Option<Payment>,
}
