//! Model for OrderLineItemItemType enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents the line item type.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemItemType {
    /// Indicates that the line item is an itemized sale.
    Item,
    /// Indicates that the line item is a non-itemized sale.
    CustomAmount,
    /// Indicates that the line item is a gift card sale. Gift cards sold through the Orders API are
    /// sold in an unactivated state and can be activated through the Gift Cards API using the line
    /// item `uid`.
    GiftCard,
}
