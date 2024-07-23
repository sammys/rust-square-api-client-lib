//! Model for CardPaymentDetailsVerificationMethod enum.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Method to verify cardholder's identity.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPaymentDetailsVerificationMethod {
    Pin,
    Signature,
    PinAndSignature,
    OnDevice,
    None,
}
