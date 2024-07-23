//! Model struct for CreateCustomerResponse type

/// This is a model struct for CreateCustomerResponse type
use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::errors::Error;
use super::Customer;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreateCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// Represents a Square customer profile in the Customer Directory of a Square seller.
    pub customer: Customer,
}
