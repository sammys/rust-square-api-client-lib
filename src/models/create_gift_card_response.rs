//! Model struct for CreateGiftCardResponse type

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, GiftCard};

/// This is a model struct for CreateGiftCardResponse type
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreateGiftCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The new gift card.
    pub gift_card: Option<GiftCard>,
}
