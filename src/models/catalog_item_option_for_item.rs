//! Model struct for CatalogItemOptionForItem type.

use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// An option that can be assigned to an item.
///
/// For example, a t-shirt item may offer a color option or a size option.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hydrate, PartialEq, Reconcile, Serialize)]
pub struct CatalogItemOptionForItem {
    /// The unique id of the item option, used to form the dimensions of the item option matrix in a
    /// specified order.
    pub item_option_id: Option<String>,
}
