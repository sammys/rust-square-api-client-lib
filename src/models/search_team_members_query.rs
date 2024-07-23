//! Model struct for SearchTeamMembersQuery type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::SearchTeamMembersFilter;

/// Represents the parameters in a search for `TeamMember` objects.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct SearchTeamMembersQuery {
    /// The options to filter by.
    pub filter: Option<SearchTeamMembersFilter>,
}
