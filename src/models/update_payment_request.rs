//! Model struct for UpdatePaymentRequest type.

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::Payment;

/// This is a model struct for UpdatePaymentRequest type.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UpdatePaymentRequest {
    /// The updated [Payment] object.
    pub payment: Option<Payment>,
    /// A unique string that identifies this `UpdatePayment` request. Keys can be any valid string
    /// but must be unique for every `UpdatePayment` request.
    ///
    /// For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/basics/api101/idempotency).
    pub idempotency_key: String,
}
