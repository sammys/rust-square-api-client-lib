//! Model struct for BatchRetrieveOrdersResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Order};

/// This is a model struct for BatchRetrieveOrdersResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct BatchRetrieveOrdersResponse {
    /// The requested orders. This will omit any requested orders that do not exist.
    pub orders: Option<Vec<Order>>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
