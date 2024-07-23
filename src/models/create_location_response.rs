//! Model struct for CreateLocationResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, Location};

/// This is a model struct for CreateLocationResponse type
#[derive(Clone, Debug, Deserialize, Hydrate, PartialEq, Reconcile)]
pub struct CreateLocationResponse {
    /// Information about [errors](https://developer.squareup.com/docs/build-basics/handling-errors)
    /// encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The newly created [Location] object.
    pub location: Option<Location>,
}
