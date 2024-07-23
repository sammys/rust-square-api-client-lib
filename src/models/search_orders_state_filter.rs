//! Model struct for SearchOrdersStateFilter type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::enums::OrderState;

/// Filter by the current order `state`.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchOrdersStateFilter {
    /// States to filter for.
    pub states: Option<Vec<OrderState>>,
}
