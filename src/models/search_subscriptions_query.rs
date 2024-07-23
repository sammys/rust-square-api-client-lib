//! Model struct for SearchSubscriptionsQuery type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::SearchSubscriptionsFilter;

/// Represents a query, consisting of specified query expressions, used to search for subscriptions.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchSubscriptionsQuery {
    /// A list of query expressions.
    pub filter: Option<SearchSubscriptionsFilter>,
}
