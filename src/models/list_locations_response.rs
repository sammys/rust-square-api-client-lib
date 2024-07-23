//! Model struct for ListLocationsResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Location};

/// This is a model struct for ListLocationsResponse type
#[derive(Clone, Debug, Deserialize, Hydrate, PartialEq, Reconcile)]
pub struct ListLocationsResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The business locations.
    pub locations: Option<Vec<Location>>,
}
