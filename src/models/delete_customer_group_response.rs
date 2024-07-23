//! Response body struct for the Delete Customer Group API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteCustomerGroupResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct DeleteCustomerGroupResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
