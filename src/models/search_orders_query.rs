//! Model struct for SearchOrdersQuery type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::{SearchOrdersFilter, SearchOrdersSort};

/// Contains query criteria for the search.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchOrdersQuery {
    /// Criteria to filter results by.
    pub filter: Option<SearchOrdersFilter>,
    /// Criteria to sort results by.
    pub sort: Option<SearchOrdersSort>,
}
