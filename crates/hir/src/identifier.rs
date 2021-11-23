//! Tydi identifiers.

use std::fmt::Display;

/// An identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl Identifier {
    /// Construct a new identifier from something that can be turned into a String.
    pub fn from<T: Into<String>>(identifier: T) -> Self {
        Self(identifier.into())
    }
}

impl From<Identifier> for String {
    fn from(i: Identifier) -> Self {
        i.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
