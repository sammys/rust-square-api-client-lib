//! Model struct for SearchOrdersSourceFilter type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

/// A filter based on order `source` information.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchOrdersSourceFilter {
    /// Filters by the [Source](OrderSource) `name`. The filter returns any orders with a `source.name` that
    /// matches any of the listed source names.
    pub source_names: Option<Vec<String>>,
}
