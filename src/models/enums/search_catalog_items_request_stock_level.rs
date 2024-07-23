//! Model for SearchCatalogItemsRequestStockLevel enum.

use crate::{Hydrate, Reconcile};
use serde::Serialize;

/// Defines supported stock levels of the item inventory.
#[derive(Clone, Debug, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub enum SearchCatalogItemsRequestStockLevel {
    /// The item inventory is empty.
    Out,
    /// The item inventory is low.
    Low,
}
