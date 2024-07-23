//! Model struct for CardPaymentTimeline type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::DateTime;

/// The timeline for card payments.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CardPaymentTimeline {
    /// The timestamp when the payment was authorized.
    pub authorized_at: Option<DateTime>,
    /// The timestamp when the payment was captured.
    pub captured_at: Option<DateTime>,
    /// The timestamp when the payment was voided.
    pub voided_at: Option<DateTime>,
}
