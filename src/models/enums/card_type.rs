//! Model for CardType enum

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Indicates a card's type, such as `CREDIT` or `DEBIT`.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    UnknownCardType,
    Credit,
    Debit,
}
