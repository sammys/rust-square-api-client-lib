//! Model struct for CatalogCustomAttributeDefinitionStringConfig type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Configuration associated with Custom Attribute Definitions of type `STRING`.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CatalogCustomAttributeDefinitionStringConfig {
    /// If true, each Custom Attribute instance associated with this Custom Attribute Definition
    /// must have a unique value within the seller's catalog. For example, this may be used for a
    /// value like a SKU that should not be duplicated within a seller's catalog. May not be
    /// modified after the definition has been created.
    pub enforce_uniqueness: Option<bool>,
}
