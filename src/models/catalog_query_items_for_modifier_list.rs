//! Model struct for CatalogQueryItemsForModifierList type.

use crate::{Hydrate, Reconcile};
use serde::Serialize;

/// The query filter to return the items containing the specified modifier list IDs.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CatalogQueryItemsForModifierList {
    /// A set of `CatalogModifierList` IDs to be used to find associated `CatalogItem`s.
    pub modifier_list_ids: Vec<String>,
}
