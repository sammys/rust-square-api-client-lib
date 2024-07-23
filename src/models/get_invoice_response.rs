//! Response body struct for the Get Invoice API.

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for GetInvoiceResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct GetInvoiceResponse {
    /// The invoice requested.
    pub refund: Option<Invoice>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
