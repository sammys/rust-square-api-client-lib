//! Model struct for UpdateLocationRequest type

use crate::{Hydrate, Reconcile};
use serde::Serialize;

use super::Location;

/// This is a model struct for the UpdateLocationRequest type
#[derive(Clone, Debug, Default, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct UpdateLocationRequest {
    /// The [Location] object with only the fields to update.
    pub location: Location,
}
