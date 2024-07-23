//! Response body struct for the Create Gift Card Activity API

use crate::{Hydrate, Reconcile};
use serde::Deserialize;

use super::{errors::Error, GiftCardActivity};

/// This is a model struct for CreateGiftCardActivityResponse type
#[derive(Clone, Debug, Deserialize, Eq, Hydrate, PartialEq, Reconcile)]
pub struct CreateGiftCardActivityResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The gift card activity that was created.
    pub gift_card_activity: Option<GiftCardActivity>,
}
