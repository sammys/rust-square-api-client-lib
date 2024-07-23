//! Response struct for the Retrieve Inventory Transfer API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, InventoryTransfer};

/// This is a model struct for RetrieveInventoryTransferResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct RetrieveInventoryTransferResponse {
    /// The requested InventoryTransfer.
    pub transfer: Option<InventoryTransfer>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
