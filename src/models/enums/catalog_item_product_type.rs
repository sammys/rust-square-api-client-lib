//! Model for CatalogItemProductType enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The type of a CatalogItem.
///
/// Connect V2 only allows the creation of `REGULAR` or `APPOINTMENTS_SERVICE` items.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemProductType {
    /// An ordinary item.
    Regular,
    /// A Square gift card.
    #[deprecated]
    GiftCard,
    /// A service that can be booked using the Square Appointments app.
    AppointmentsService,
}
