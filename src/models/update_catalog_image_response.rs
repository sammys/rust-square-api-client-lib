//! Response struct for the Update Catalog Image API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, CatalogObject};

/// This is a model struct for UpdateCatalogImageResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct UpdateCatalogImageResponse {
    /// The newly updated `CatalogImage` including a Square-generated URL for the encapsulated image
    /// file.
    pub image: CatalogObject,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
