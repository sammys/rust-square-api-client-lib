//! Model struct for UpdateLocationResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Location};

/// This is a model struct for UpdateLocationResponse type
#[derive(Clone, Debug, Deserialize, Hydrate, PartialEq, Reconcile)]
pub struct UpdateLocationResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated [Location] object.
    pub location: Option<Location>,
}
