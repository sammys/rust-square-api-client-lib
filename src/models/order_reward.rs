//! Model struct for OrderReward type

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// Represents a reward that can be applied to an order if the necessary reward tier criteria are met.
///
/// Rewards are created through the Loyalty API.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct OrderReward {
    /// The identifier of the reward.
    pub id: String,
    /// The identifier of the reward tier corresponding to this reward.
    pub reward_tier_id: String,
}
