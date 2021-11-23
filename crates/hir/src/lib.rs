//! The Tydi High-level Intermediate Representation.

pub mod component;
pub mod identifier;
pub mod net;
pub mod r#type;

pub use crate::{
    component::{Component, Instance},
    identifier::Identifier,
    net::{Connection, InstancePort, Mode, Net, Port, Wire},
    r#type::*,
};
use std::sync::Arc;
use tydi_intern::Id;

/// The root node of a hardware design description.
// TODO: When we allow importing external projects, we need to change this.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Project {
    /// The modules within a project.
    pub modules: Vec<Id<Module>>,
}

/// A collection of components, constants and types.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Module {
    /// The identifier of the module.
    pub identifier: Id<Identifier>,
    /// The components that this module describes.
    pub components: Vec<Id<Component>>,
    /// The types that this module describes.
    pub types: Vec<Id<Type>>,
    // TODO: constants
    // TODO: nested modules
}

impl Module {
    /// Create an empty module.
    pub fn new(identifier: Id<Identifier>) -> Self {
        Self {
            identifier,
            components: vec![],
            types: vec![],
        }
    }

    /// Append a component to the module.
    pub fn with_component(mut self, comp: Id<Component>) -> Self {
        self.components.push(comp);
        self
    }

    /// Append a type to the module.
    pub fn with_type(mut self, typ: Id<Type>) -> Self {
        self.types.push(typ);
        self
    }
}

#[salsa::query_group(HirStorage)]
pub trait Hir {
    #[salsa::input]
    fn project(&self) -> Arc<Project>;

    #[salsa::interned]
    fn intern_module(&self, modules: Module) -> Id<Module>;
    #[salsa::interned]
    fn intern_component(&self, component: Component) -> Id<Component>;
    #[salsa::interned]
    fn intern_instance(&self, instance: Instance) -> Id<Instance>;
    #[salsa::interned]
    fn intern_identifier(&self, identifier: Identifier) -> Id<Identifier>;
    #[salsa::interned]
    fn intern_port(&self, port: Port) -> Id<Port>;
    #[salsa::interned]
    fn intern_net(&self, net: Net) -> Id<Net>;
    #[salsa::interned]
    fn intern_instance_port(&self, instance_port: InstancePort) -> Id<InstancePort>;
    #[salsa::interned]
    fn intern_connection(&self, connection: Connection) -> Id<Connection>;
    #[salsa::interned]
    fn intern_type(&self, typ: Type) -> Id<Type>;
    #[salsa::interned]
    fn intern_logical_type(&self, typ: LogicalType) -> Id<LogicalType>;
    #[salsa::interned]
    fn intern_field(&self, field: Field) -> Id<Field>;

    /// Obtain all modules.
    fn modules(&self) -> Arc<Vec<Id<Module>>>;

    /// Obtain all components from a modules.
    fn components(&self, modules: Id<Module>) -> Arc<Vec<Id<Component>>>;

    /// Obtain all instances of a component.
    fn instances(&self, component: Id<Component>) -> Arc<Vec<Id<Instance>>>;

    /// Obtain all nets of a component.
    fn nets(&self, component: Id<Component>) -> Arc<Vec<Net>>;

    /// Obtain all Port nets of a component.
    fn ports(&self, component: Id<Component>) -> Arc<Vec<Port>>;

    /// Obtain all Wire nets of a component.
    fn wires(&self, component: Id<Component>) -> Arc<Vec<Wire>>;

    /// Obtain all InstancePort nets of a component.
    fn instance_ports(&self, component: Id<Component>) -> Arc<Vec<InstancePort>>;

    /// Get the identifier of a modules.
    fn module_identifier(&self, modules: Id<Module>) -> Arc<Identifier>;

    /// Get the identifier of a component.
    fn component_identifier(&self, component: Id<Component>) -> Arc<Identifier>;

    /// Get the identifier of a port.
    fn port_identifier(&self, component: Id<Port>) -> Arc<Identifier>;

    /// Get the identifier of an instance.
    fn instance_identifier(&self, component: Id<Instance>) -> Arc<Identifier>;
}

fn modules(db: &dyn Hir) -> Arc<Vec<Id<Module>>> {
    Arc::new(db.project().modules.clone())
}

fn components(db: &dyn Hir, module: Id<Module>) -> Arc<Vec<Id<Component>>> {
    Arc::new(db.lookup_intern_module(module).components)
}

fn instances(db: &dyn Hir, component: Id<Component>) -> Arc<Vec<Id<Instance>>> {
    Arc::new(db.lookup_intern_component(component).instances)
}

fn nets(db: &dyn Hir, component: Id<Component>) -> Arc<Vec<Net>> {
    Arc::new(
        db.lookup_intern_component(component)
            .nets
            .iter()
            .map(|&nid| db.lookup_intern_net(nid))
            .collect(),
    )
}

fn ports(db: &dyn Hir, component: Id<Component>) -> Arc<Vec<Port>> {
    Arc::new(
        db.nets(component)
            .iter()
            .filter_map(|net| match *net {
                Net::Port(p) => Some(db.lookup_intern_port(p)),
                _ => None,
            })
            .collect(),
    )
}

fn wires(db: &dyn Hir, component: Id<Component>) -> Arc<Vec<Wire>> {
    Arc::new(
        db.nets(component)
            .iter()
            .filter_map(|net| match *net {
                Net::Wire(w) => Some(w),
                _ => None,
            })
            .collect(),
    )
}

fn instance_ports(db: &dyn Hir, component: Id<Component>) -> Arc<Vec<InstancePort>> {
    Arc::new(
        db.nets(component)
            .iter()
            .filter_map(|net| match *net {
                Net::InstancePort(i) => Some(i),
                _ => None,
            })
            .collect(),
    )
}

// TODO: make a macro for everything that has an identifier:
fn module_identifier(db: &dyn Hir, modules: Id<Module>) -> Arc<Identifier> {
    Arc::new(db.lookup_intern_identifier(db.lookup_intern_module(modules).identifier))
}

fn component_identifier(db: &dyn Hir, component: Id<Component>) -> Arc<Identifier> {
    Arc::new(db.lookup_intern_identifier(db.lookup_intern_component(component).identifier))
}

fn port_identifier(db: &dyn Hir, port: Id<Port>) -> Arc<Identifier> {
    Arc::new(db.lookup_intern_identifier(db.lookup_intern_port(port).identifier))
}

fn instance_identifier(db: &dyn Hir, instance: Id<Instance>) -> Arc<Identifier> {
    Arc::new(db.lookup_intern_identifier(db.lookup_intern_instance(instance).identifier))
}
