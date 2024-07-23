//! Model for CatalogPricingType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Indicates whether the price of a CatalogItemVariation should be entered manually at the time of
/// sale.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogPricingType {
    /// The catalog item variation's price is fixed.
    FixedPricing,
    /// The catalog item variation's price is entered at the time of sale.
    VariablePricing,
}
