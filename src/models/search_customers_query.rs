//! Model struct for SearchCustomersQuery type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::{SearchCustomersFilter, SearchCustomersSort};

/// Contains query criteria for the search.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchCustomersQuery {
    /// Criteria to filter results by.
    pub filter: Option<SearchCustomersFilter>,
    /// Criteria to sort results by.
    pub sort: Option<SearchCustomersSort>,
}
