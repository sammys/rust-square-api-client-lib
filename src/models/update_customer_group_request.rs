//! Request body struct for the Update Customer Group API

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::CustomerGroup;

/// This is a model struct for UpdateCustomerGroupRequest type.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UpdateCustomerGroupRequest {
    /// The [CustomerGroup] object including all the updates you want to make.
    pub group: CustomerGroup,
}
