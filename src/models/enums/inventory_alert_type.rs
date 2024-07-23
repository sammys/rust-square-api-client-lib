//! Model for InventoryAlertType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Indicates whether Square should alert the merchant when the inventory quantity of a
/// CatalogItemVariation is low.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryAlertType {
    /// The variation does not display an alert.
    None,
    /// The variation generates an alert when its quantity is low.
    LowQuantity,
}
