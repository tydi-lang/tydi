//! A user-friendly Rust API to construct a Tydi High-level Intermediate
//! Representation programmatically.
#![warn(missing_docs)]

use tydi_hir::{Hir, Identifier};
use tydi_intern::Id;

/// Intern an identifier and return its id.
pub fn ident<T: Into<String>>(db: &dyn Hir, identifier: T) -> Id<Identifier> {
    db.intern_identifier(Identifier::from(identifier))
}
