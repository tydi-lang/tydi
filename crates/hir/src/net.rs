//! Tydi nets (e.g. wires and ports) and connections between them.

use tydi_intern::Id;

use crate::{component::Instance, identifier::Identifier, r#type::Type};

/// An I/O mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// An input.
    Input,
    /// An output.
    Output,
}

/// A wire inside a component.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Wire {
    /// The identifier of the wire.
    pub identifier: Id<Identifier>,
    /// The type of the wire.
    pub typ: Id<Type>,
    // TODO: default driver
}

/// A port of a component.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Port {
    /// The identifier of the port.
    pub identifier: Id<Identifier>,
    /// The mode of the port.
    pub mode: Mode,
    /// The type of the port.
    pub typ: Id<Type>,
    // TODO: default driver
}

/// A port of an instance.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstancePort {
    /// The instance.
    pub instance: Id<Instance>,
    /// The port of the instance.
    pub port: Id<Port>,
}

impl InstancePort {
    /// Create a new InstancePort.
    pub fn new(instance: Id<Instance>, port: Id<Port>) -> Self {
        Self { instance, port }
    }
}

/// A net that is sinked and sourced.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Net {
    /// A port of the component itself.
    Port(Id<Port>),
    /// A wire inside the component.
    Wire(Wire),
    /// The port of an instance.
    InstancePort(InstancePort),
}

/// A connection between two nets.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Connection {
    /// The sourcing net.
    pub source: Id<Net>,
    /// The sinking net.
    pub sink: Id<Net>,
}

impl Connection {
    /// Create a new connection between source and sink.
    pub fn new(source: Id<Net>, sink: Id<Net>) -> Self {
        Self { source, sink }
    }
}
