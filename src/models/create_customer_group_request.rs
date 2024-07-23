//! Request body struct for the Create Customer Group API

/// This is a model struct for CreateCustomerGroupRequest type
use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::CustomerGroup;

#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CreateCustomerGroupRequest {
    /// The idempotency key for the request. For more information, see
    /// [Idempotency](https://developer.squareup.com/docs/working-with-apis/idempotency).
    pub idempotency_key: Option<String>,
    /// The customer group to create.
    pub group: CustomerGroup,
}
