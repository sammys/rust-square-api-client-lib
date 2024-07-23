//! Model struct for DateTime type.

use std::fmt::{Debug, Display};

use chrono::Utc;
use log::error;
use crate::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use super::errors::ApiError;

/// Represents a Timestamp or DateTime.
///
/// Handles Serialization and Deserialization and conversion between Square standard
/// RFC3339 String and Unix Timestamp i64.
///
/// Uses chrono library under the hood.
#[derive(Clone, Eq, Hydrate, PartialEq, Reconcile)]
pub struct DateTime {
    #[cfg(feature="automerge")]
    #[autosurgeon(hydrate="hydrate_datetime",reconcile="reconcile_datetime")]
    inner: chrono::DateTime<Utc>,

    #[cfg(not(feature="automerge"))]
    inner: chrono::DateTime<Utc>,
}

#[cfg(feature="automerge")]
fn hydrate_datetime<'a, D: autosurgeon::ReadDoc>(
    doc: &D,
    obj: &automerge::ObjId,
    prop: autosurgeon::Prop<'a>,
) -> Result<chrono::DateTime<Utc>, autosurgeon::HydrateError> {
    let inner = i64::hydrate(doc, obj, prop)?;
    // let inner = String::hydrate(doc, obj, prop)?;
    chrono::DateTime::from_timestamp_micros(inner)
        .ok_or_else(|| autosurgeon::HydrateError::unexpected(
            "a valid i64 timestamp", format!("a DateTime<Utc> which failed to parse: {}", inner)
        ))
}

#[cfg(feature="automerge")]
fn reconcile_datetime<R: autosurgeon::Reconciler>(
    datetime: &chrono::DateTime<Utc>, mut reconciler: R
) -> Result<(), R::Error> {
    reconciler.i64(datetime.timestamp_micros())
}

impl DateTime {
    /// Generates a new `DateTime` representing the present time.
    ///
    /// Alias for `DateTime::now()`
    pub fn new() -> Self {
        Self::now()
    }

    /// Generates a new `DateTime` representing the present time.
    ///
    /// Alias for `DateTime::new()`
    pub fn now() -> Self {
        Self {
            inner: chrono::offset::Utc::now(),
        }
    }
}

impl Debug for DateTime {
    /// Unwraps the inner DateTime for Debug.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl Default for DateTime {
    /// Default `DateTime` is the present/current time.
    ///
    /// Alias for `DateTime::now()`
    fn default() -> Self {
        Self::now()
    }
}

impl<'de> Deserialize<'de> for DateTime {
    /// Attempts to generate a `DateTime` from its serialized version.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: chrono::DateTime::deserialize(deserializer)?,
        })
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl From<&chrono::DateTime<Utc>> for DateTime {
    /// Converts a `chrono::DateTime<Utc>` reference into a `DateTime`.
    fn from(dt: &chrono::DateTime<Utc>) -> Self {
        Self {
            inner: dt.to_owned(),
        }
    }
}

impl From<i64> for DateTime {
    /// Converts a Unix timestamp into a `DateTime`.
    fn from(timestamp: i64) -> Self {
        Self::from(&chrono::DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(timestamp, 0),
            Utc,
        ))
    }
}

impl Serialize for DateTime {
    /// Unwraps the inner DateTime for Serialization.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl TryFrom<&str> for DateTime {
    type Error = ApiError;

    /// Attempts to generate a `DateTime` from RFC3339 formatted String slice.
    ///
    /// Returns an API Error if the input String slice cannot be parsed.
    fn try_from(rfc3339: &str) -> Result<Self, Self::Error> {
        let inner = chrono::DateTime::parse_from_rfc3339(rfc3339).map_err(|e| {
            let msg = format!("Error parsing RFC3339 DateTime string: {}: {}", rfc3339, e);
            error!("{}", &msg);
            Self::Error::new(&msg)
        })?;
        let inner = inner.with_timezone(&chrono::offset::Utc);
        Ok(Self { inner })
    }
}
