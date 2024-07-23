//! Model for CardPrepaidType enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Indicates a card's prepaid type, such as `NOT_PREPAID` or `PREPAID`.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPrepaidType {
    UnknownPrepaidType,
    NotPrepaid,
    Prepaid,
}
