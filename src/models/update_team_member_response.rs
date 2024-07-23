//! Response body struct for the Update Team Member API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, TeamMember};

/// This is a model struct for UpdateTeamMemberResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct UpdateTeamMemberResponse {
    /// The successfully updated `TeamMember` object.
    pub team_member: Option<TeamMember>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
