//! Response body struct for the Update Wage Setting API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, WageSetting};

/// This is a model struct for UpdateWageSettingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct UpdateWageSettingResponse {
    /// The successfully updated `WageSetting` object.
    pub wage_setting: Option<WageSetting>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
