//! Response body struct for the Create Invoice API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Invoice};

/// This is a model struct for CreateInvoiceResponse type
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreateInvoiceResponse {
    /// The newly created invoice.
    pub invoice: Option<Invoice>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
