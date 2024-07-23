//! Request body struct for the Update Team Member API

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::TeamMember;

/// This is a model struct for UpdateTeamMemberRequest type.
#[derive(Clone, Debug, Default, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UpdateTeamMemberRequest {
    /// The data used to update the `TeamMember` object.
    pub team_member: Option<TeamMember>,
}
