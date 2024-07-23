//! Model struct for OrderSource type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents the origination details of an order.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct OrderSource {
    /// The name used to identify the place (physical or digital) that an order originates. If
    /// unset, the name defaults to the name of the application that created the order.
    pub name: Option<String>,
}
