//! Model for CardPaymentDetailsEntryMethod enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Method used to enter a card's details for payment.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsEntryMethod {
    Keyed,
    Swiped,
    Emv,
    OnFile,
    Contactless,
}
