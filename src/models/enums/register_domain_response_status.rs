//! Model for RegisterDomainResponseStatus enum

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

/// The status of the domain registration.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegisterDomainResponseStatus {
    /// The domain is added, but not verified.
    Pending,
    /// The domain is added and verified. It can be used to accept Apple Pay transactions.
    Verified,
}
