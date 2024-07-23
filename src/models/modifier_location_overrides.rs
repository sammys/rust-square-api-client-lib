//! Model struct for ModifierLocationOverrides type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::Money;

/// Price and inventory alerting overrides for a `CatalogItemVariation` at a specific `Location`.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct ModifierLocationOverrides {
    /// The ID of the `Location`. This can include locations that are deactivated.
    pub location_id: Option<String>,
    /// The price of the `CatalogItemVariation` at the given `Location`, or blank for variable
    /// pricing.
    pub price_money: Option<Money>,
}
