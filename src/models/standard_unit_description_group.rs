//! Model struct for StandardUnitDescriptionGroup

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::StandardUnitDescription;

/// Group of standard measurement units.
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct StandardUnitDescriptionGroup {
    /// List of standard (non-custom) measurement units in this description group.
    pub standard_unit_descriptions: Option<Vec<StandardUnitDescription>>,
    /// IETF language tag.
    pub language_code: Option<String>,
}
