//! Tydi components.

use tydi_intern::Id;

use crate::{
    identifier::Identifier,
    net::{Connection, Net},
    Type,
};

/// A component. Represents a hierarchical entity in a hardware design.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Component {
    // TODO: generics
    // TODO: implementation
    /// The identifier of the component.
    pub identifier: Id<Identifier>,
    /// The instances within a component.
    pub instances: Vec<Id<Instance>>,
    /// The nets of a component, including its ports.
    pub nets: Vec<Id<Net>>,
    /// The connections within a component.
    pub connections: Vec<Connection>,
    /// The types used within a component.
    pub types: Vec<Id<Type>>,
    /// Component metadata.
    // We want a map here, but HashMap doesn't implement Hash.
    // BTreeMap does, which we could consider using if performance of lookup ever becomes a problem.
    // see: https://internals.rust-lang.org/t/implementing-hash-for-hashset-hashmap/3817
    // and: https://github.com/rust-lang/rust/pull/48366
    pub metadata: Vec<(String, String)>,
}

impl Component {
    /// Create an empty component.
    pub fn new(identifier: Id<Identifier>) -> Self {
        Self {
            identifier,
            instances: vec![],
            nets: vec![],
            connections: vec![],
            metadata: vec![],
            types: vec![],
        }
    }

    /// Append an instance to the component.
    pub fn with_instance(mut self, instance: Id<Instance>) -> Self {
        self.instances.push(instance);
        self
    }

    /// Append a net to the component.
    pub fn with_net(mut self, net: Id<Net>) -> Self {
        self.nets.push(net);
        self
    }

    /// Append a connection to the component.
    pub fn with_connection(mut self, connection: Connection) -> Self {
        self.connections.push(connection);
        self
    }

    /// Append key-value metadata to the component.
    pub fn with_metadata<T: Into<String>>(mut self, key: T, value: T) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}

/// An instance of a component.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Instance {
    /// The identifier of the instance.
    pub identifier: Id<Identifier>,
    /// The component that is instantiated.
    pub component: Id<Component>,
}

impl Instance {
    /// Create a new instance.
    pub fn new(identifier: Id<Identifier>, component: Id<Component>) -> Self {
        Self {
            identifier,
            component,
        }
    }
}
