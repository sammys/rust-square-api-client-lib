pub mod api;
pub mod config;
pub mod http;
pub mod models;
mod square_client;

pub use square_client::SquareClient;

#[cfg(feature = "automerge")]
pub use autosurgeon::{Hydrate, Reconcile};

#[cfg(not(feature = "automerge"))]
pub use mock_autosurgeon::{Hydrate, Reconcile};
