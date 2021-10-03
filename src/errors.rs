//! Error types (non-FFXIV)

use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error representing an unknown variant of any `enum`.
///
/// `(enum name, unknown variant)`, e.g. `("DataCenter", "my invalid input")`
///
/// This is generally encountered when using [`FromStr`] on any `enum` in this crate.
///
/// [`FromStr`]: ::std::str::FromStr
#[derive(Debug)]
pub struct UnknownVariant(
    /// The `enum` name (e.g. `"DataCenter"`)
    pub &'static str,
    /// The unknown variant given
    pub String,
);

impl Display for UnknownVariant {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "unknown variant {} for type {}", self.1, self.0)
    }
}
