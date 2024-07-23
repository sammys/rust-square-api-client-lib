//! Model struct for DeleteCustomerResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteCustomerResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct DeleteCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
