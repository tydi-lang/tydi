//! Tydi type system.
use std::num::NonZeroUsize;

use tydi_intern::Id;

use crate::identifier::Identifier;

/// A type with a name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamedType {
    /// The name of the type.
    pub identifier: Id<Identifier>,
    /// The type definition
    pub typ: Id<LogicalType>,
}

impl NamedType {
    /// Create a new NamedType.
    pub fn new(identifier: Id<Identifier>, typ: Id<LogicalType>) -> Self {
        Self { identifier, typ }
    }
}

/// The direction of things that have directions, such as fields in compound types
/// like Group or Union.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Forward direction (default).
    Forward,
    /// Reverse direction.
    Reverse,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Forward
    }
}

/// A Tydi Type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    /// A named type.
    Named(NamedType),
    /// An anonymous type.
    Anon(Id<LogicalType>),
}

/// The field of a group or union.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Field {
    /// Field name.
    pub identifier: Id<Identifier>,
    /// Reference to the field type.
    pub typ: Id<Type>,
    /// Direction of the field w.r.t. whatever uses its parent type.
    pub direction: Direction,
}

impl Field {
    /// Create a new field.
    pub fn new(identifier: Id<Identifier>, typ: Id<Type>, direction: Direction) -> Self {
        Self {
            identifier,
            typ,
            direction,
        }
    }
}

/// The Group type.
///
/// This is a product type holding multiple fields at the same time.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Group {
    /// The fields of the Group.
    pub fields: Vec<Id<Field>>,
}

impl Group {
    /// Create a new Group type.
    pub fn new(fields: Vec<Id<Field>>) -> Self {
        Self { fields }
    }
}

/// The Tydi Logical Type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogicalType {
    /// The Null type with single-valued data which is always null.
    Null,
    /// The Bits type representing a positive number of bits.
    Bits(NonZeroUsize),
    /// The Group type representing a product type of multiple fields.
    Group(Group),
}
