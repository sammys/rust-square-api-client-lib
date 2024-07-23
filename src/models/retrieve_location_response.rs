//! Model struct for RetrieveLocationResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Location};

/// This is a model struct for RetrieveLocationResponse type
#[derive(Clone, Debug, Deserialize, Hydrate, PartialEq, Reconcile)]
pub struct RetrieveLocationResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested location.
    pub location: Option<Location>,
}
