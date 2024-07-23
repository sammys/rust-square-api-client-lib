//! Model struct for CancelPaymentByIdempotencyKeyResponse type.

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for CancelPaymentByIdempotencyKeyResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CancelPaymentByIdempotencyKeyResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
