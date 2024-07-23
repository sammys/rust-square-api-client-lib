//! Model struct for CalculateOrderResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Order};

/// This is a model struct for CalculateOrderResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CalculateOrderResponse {
    /// The calculated version of the order provided in the request.
    pub order: Order,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
