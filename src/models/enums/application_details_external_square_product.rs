//! Model for ApplicationDetailsExternalSquareProduct enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// A list of products to return to external callers.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationDetailsExternalSquareProduct {
    Appointments,
    EcommerceApi,
    Invoices,
    OnlineStore,
    Other,
    Restaurants,
    Retail,
    SquarePos,
    TerminalApi,
    VirtualTerminal,
}
