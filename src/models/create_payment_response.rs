//! Response struct for the Create Payment API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::errors::Error;
use super::Payment;

/// This is a model struct for CreatePaymentResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreatePaymentResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created payment.
    pub payment: Payment,
}
