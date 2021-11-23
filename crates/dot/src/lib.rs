//! A DOT back-end to visualize the High-level Intermediate Representation.

#![warn(missing_docs)]

mod node;
pub mod style;

use node::{draw_edge, DotDisplay, DotNode, DotNodeId};
use std::fmt::{Display, Formatter, Result};
use style::Style;
use tydi_db::Database;
use tydi_hir::{
    Component, Connection, Direction, Field, Hir, Identifier, Instance, LogicalType, Mode, Module,
    Net, Port, Type,
};
use tydi_intern::Id;

/// Trait to traverse through High-level Intermediate Representation nodes and
/// produce a DOT source.
pub trait Dot {
    /// Write the DOT representation of self to a formatter.
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result;
}

impl DotDisplay for Identifier {
    fn to_dot(&self) -> String {
        // TODO: sanitize the string for DOT
        self.to_string()
    }
}

/// Macro to quickly implement the DotNodeId trait for Id types where the resulting
/// DOT node id is prefixed to disambiguate them from other types of Ids.
macro_rules! dot_id {
    ($typ:ty, $prefix:literal) => {
        impl DotNodeId for Id<$typ> {
            fn to_dot_node_id(&self) -> String {
                format!("{}_{}", $prefix, self)
            }
        }

        impl DotNodeId for &Id<$typ> {
            fn to_dot_node_id(&self) -> String {
                format!("{}_{}", $prefix, self)
            }
        }
    };
}

// DOT node prefixes for database IDs
dot_id!(Instance, "ins");
dot_id!(Component, "cmp");
dot_id!(Module, "mdl");
dot_id!(Net, "net");
dot_id!(Type, "typ");
dot_id!(LogicalType, "lty");
dot_id!(Field, "fld");
dot_id!(Port, "prt");

impl DotDisplay for Direction {
    fn to_dot(&self) -> String {
        match self {
            Direction::Forward => "▶",
            Direction::Reverse => "◀",
        }
        .to_string()
    }
}

impl DotDisplay for Mode {
    fn to_dot(&self) -> String {
        match self {
            Mode::Input => "🄸",
            Mode::Output => "🄾",
        }
        .to_string()
    }
}

impl Dot for Id<Field> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let field = db.lookup_intern_field(*self);
        DotNode::new(&style.nodes.field, self)
            .with_label(format!(
                "{} {}",
                field.direction.to_dot(),
                db.lookup_intern_identifier(field.identifier)
            ))
            .fmt(f)?;
        draw_edge(f, self, field.typ, style.edges.reference, None)
    }
}

impl Dot for Id<LogicalType> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let lt = db.lookup_intern_logical_type(*self);
        DotNode::new(&style.nodes.logical_type, self)
            .with_label(match lt {
                LogicalType::Null => "Null".to_string(),
                LogicalType::Bits(b) => format!("Bits[{}]", b),
                LogicalType::Group(_) => "Group".to_string(),
            })
            .fmt(f)?;
        if let LogicalType::Group(g) = lt {
            for field in g.fields {
                draw_edge(f, self, field, style.edges.child, None)?;
                field.dottify(f, db, style)?;
            }
        }
        Ok(())
    }
}

impl Dot for Id<Type> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let typ = db.lookup_intern_type(*self);
        DotNode::new(&style.nodes.typ, self)
            .with_label(match &typ {
                Type::Named(nt) => db.lookup_intern_identifier(nt.identifier).to_string(),
                Type::Anon(_) => "[anon]".to_string(),
            })
            .fmt(f)?;
        let lt = match typ {
            Type::Named(nt) => nt.typ,
            Type::Anon(lt) => lt,
        };
        draw_edge(f, self, lt, style.edges.child, None)?;
        lt.dottify(f, db, style)
    }
}

impl Dot for Id<Port> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let port = db.lookup_intern_port(*self);
        DotNode::new(&style.nodes.port, self)
            .with_label(format!(
                "{} {}",
                port.mode.to_dot(),
                db.lookup_intern_identifier(port.identifier),
            ))
            .fmt(f)?;
        draw_edge(f, self, port.typ, style.edges.reference, None)
    }
}

impl Dot for Id<Net> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let net = db.lookup_intern_net(*self);
        match net {
            Net::Port(port_id) => {
                DotNode::new(&style.nodes.net_port, self).fmt(f)?;
                draw_edge(f, self, port_id, &style.edges.child, None)?;
                port_id.dottify(f, db, style)
            }
            Net::Wire(wire) => {
                DotNode::new(&style.nodes.wire, self)
                    .with_label(db.lookup_intern_identifier(wire.identifier))
                    .fmt(f)?;
                draw_edge(f, self, wire.typ, &style.edges.reference, None)
            }
            Net::InstancePort(inst_port) => {
                let inst = db.lookup_intern_instance(inst_port.instance);
                let port = db.lookup_intern_port(inst_port.port);
                DotNode::new(&style.nodes.instance_port, self)
                    .with_label(format!(
                        "{}.{}",
                        db.lookup_intern_identifier(inst.identifier),
                        db.lookup_intern_identifier(port.identifier)
                    ))
                    .fmt(f)?;
                draw_edge(f, self, inst_port.instance, style.edges.reference, None)?;
                draw_edge(f, self, inst_port.port, style.edges.reference, None)
            }
        }
    }
}

impl DotNodeId for &Connection {
    fn to_dot_node_id(&self) -> String {
        format!(
            "{}_{}",
            self.source.to_dot_node_id(),
            self.sink.to_dot_node_id()
        )
    }
}

impl Dot for Connection {
    fn dottify(&self, f: &mut Formatter, _: &Database, style: &Style) -> Result {
        DotNode::new(&style.nodes.connection, self).fmt(f)?;
        draw_edge(
            f,
            self,
            self.source,
            style.edges.reference,
            Some("src".to_string()),
        )?;
        draw_edge(
            f,
            self,
            self.sink,
            style.edges.reference,
            Some("dst".to_string()),
        )
    }
}

impl Dot for Id<Instance> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let instance = db.lookup_intern_instance(*self);
        DotNode::new(&style.nodes.instance, self)
            .with_label(db.lookup_intern_identifier(instance.identifier))
            .fmt(f)?;
        draw_edge(f, self, instance.component, style.edges.reference, None)
    }
}

impl Dot for Id<Component> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let component = db.lookup_intern_component(*self);
        DotNode::new(&style.nodes.component, self)
            .with_label(db.lookup_intern_identifier(component.identifier))
            .fmt(f)?;
        for instance in component.instances {
            draw_edge(f, self, instance, style.edges.child, None)?;
            instance.dottify(f, db, style)?;
        }
        for net in component.nets {
            draw_edge(f, self, net, style.edges.child, None)?;
            net.dottify(f, db, style)?;
        }
        for connection in component.connections {
            draw_edge(f, self, &connection, style.edges.child, None)?;
            connection.dottify(f, db, style)?;
        }
        Ok(())
    }
}

impl Dot for Id<Module> {
    fn dottify(&self, f: &mut Formatter, db: &Database, style: &Style) -> Result {
        let module = db.lookup_intern_module(*self);
        DotNode::new(&style.nodes.module, self)
            .with_label(db.lookup_intern_identifier(module.identifier))
            .fmt(f)?;
        for c in module.components {
            draw_edge(f, self, c, style.edges.child.to_string(), None)?;
            c.dottify(f, db, style)?;
        }
        for t in module.types {
            draw_edge(f, self, t, style.edges.child.to_string(), None)?;
            t.dottify(f, db, style)?;
        }
        Ok(())
    }
}

/// A Graphviz DOT graph representation of a high-level intermediate representation.
pub struct DotProject<'a> {
    db: Database,
    style: Style<'a>,
}

impl<'a> DotProject<'a> {
    /// Create a new DotProject.
    pub fn new(db: Database, style: Style<'a>) -> Self {
        Self { db, style }
    }
}

impl<'a> Display for DotProject<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let style = Style::default();
        writeln!(f, "digraph Project {{",)?;
        //writeln!(f, "splines=ortho;")?;

        DotNode::new(&style.nodes.root, "root").fmt(f)?;

        for m in self.db.modules().iter() {
            draw_edge(f, "root", m, self.style.edges.child.to_string(), None)?;
            m.dottify(f, &self.db, &self.style)?;
        }

        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Result};

    // TODO: make better tests.

    #[test]
    fn types() -> Result<()> {
        let graph = DotProject {
            db: tydi_designs::types(),
            style: Style::default(),
        };
        let dot = format!("{}\n", graph);
        fs::write("dot.dot", dot)
    }

    #[test]
    fn duplicated_components() -> Result<()> {
        let graph = DotProject {
            db: tydi_designs::duplicated_components(),
            style: Style::default(),
        };
        let dot = format!("{}\n", graph);
        fs::write("dot.dot", dot)
    }

    #[test]
    fn component_loop() -> Result<()> {
        let graph = DotProject {
            db: tydi_designs::component_loop(),
            style: Style::default(),
        };
        let dot = format!("{}\n", graph);
        fs::write("dot.dot", dot)
    }
}
