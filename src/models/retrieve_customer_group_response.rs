//! Response body struct for the Retrieve Customer Group API

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::{errors::Error, CustomerGroup};

/// This is a model struct for RetrieveCustomerGroupResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct RetrieveCustomerGroupResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The retrieved customer group.
    pub group: CustomerGroup,
}
