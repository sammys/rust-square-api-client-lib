//! Model struct for InvoiceSort type.

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::enums::{InvoiceSortField, SortOrder};

/// Identifies the sort field and sort order.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct InvoiceSort {
    /// The field to use for sorting.
    pub field: InvoiceSortField,
    /// The order to use for sorting the results.
    pub order: Option<SortOrder>,
}
