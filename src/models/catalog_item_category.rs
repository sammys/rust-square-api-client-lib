//! Model struct for CatalogV1Id type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// A Square API V1 identifier of an item, including the object ID and its associated location ID.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CatalogItemCategory {
    /// The ID of the item's category, if any.
    pub id: Option<String>,
    /// The ID of the `Location` this Connect V1 ID is associated with.
    pub ordinal: Option<i64>,
}
