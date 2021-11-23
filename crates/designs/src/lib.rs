//! Pre-defined Tydi High-Level Intermediate representation designs for debugging
//! purposes.

#![warn(missing_docs)]

use std::{convert::TryFrom, num::NonZeroUsize, sync::Arc};

use tydi_db::Database;
use tydi_hcl::ident;
use tydi_hir::{
    Component, Connection, Direction, Field, Group, Hir, Instance, InstancePort, LogicalType, Mode,
    Module, NamedType, Net, Port, Project, Type,
};

/// Return a design with various type configurations.
pub fn types() -> Database {
    let mut db = Database::default();

    let mod_id = ident(&db, "module");
    let t0_id = ident(&db, "q");
    let t1_id = ident(&db, "r");
    let t2_id = ident(&db, "s");

    let t0 = db.intern_type(Type::Named(NamedType::new(
        t0_id,
        db.intern_logical_type(LogicalType::Null),
    )));

    let t1 = db.intern_type(Type::Named(NamedType::new(
        t1_id,
        db.intern_logical_type(LogicalType::Bits(NonZeroUsize::try_from(1).unwrap())),
    )));

    let t2 = db.intern_type(Type::Named(NamedType::new(
        t2_id,
        db.intern_logical_type(LogicalType::Group(Group::new(vec![
            db.intern_field(Field::new(ident(&db, "a"), t0, Direction::Forward)),
            db.intern_field(Field::new(ident(&db, "b"), t0, Direction::Reverse)),
        ]))),
    )));

    let module = db.intern_module(Module {
        identifier: mod_id,
        components: vec![],
        types: vec![t0, t1, t2],
    });

    let project = Project {
        modules: vec![module],
    };

    db.set_project(Arc::new(project));
    db
}

/// Returns a design with two packages with one component each, where both of
/// these components are exactly the same. In this stage of the project, these
/// components will be primitive, e.g. they will not have an implementation,
/// which is typically left to a user after template generation for the
/// component. However, Salsa deduplicates stuff in the database, so we need to
/// make sure we can still discern between component a.x and b.x, even though x is
/// seemingly identical and Salsa deduplicates both x-es. We currently prevent
/// deduplication by inserting some metadata that is not really needed by
/// back-ends.
pub fn duplicated_components() -> Database {
    let mut db = Database::default();

    let comp_0 = db.intern_component(Component::new(ident(&db, "x")).with_metadata("parent", "a"));
    let comp_1 = db.intern_component(Component::new(ident(&db, "x")).with_metadata("parent", "b"));
    let mod_0 = db.intern_module(Module::new(ident(&db, "a")).with_component(comp_0));
    let mod_1 = db.intern_module(Module::new(ident(&db, "b")).with_component(comp_1));

    let project = Project {
        modules: vec![mod_0, mod_1],
    };

    db.set_project(Arc::new(project));

    db
}

/// Returns a design with one package with two components. The "top" component
/// instantiates the "a" component twice, and connects an output "o" to an
/// input "i" for both instances.
pub fn component_loop() -> Database {
    let mut db = Database::default();

    let typ = db.intern_type(Type::Anon(
        db.intern_logical_type(LogicalType::Bits(NonZeroUsize::try_from(1).unwrap())),
    ));

    let port_in = db.intern_port(Port {
        identifier: ident(&db, "i"),
        mode: Mode::Input,
        typ,
    });
    let net_port_in = db.intern_net(Net::Port(port_in));

    let port_out = db.intern_port(Port {
        identifier: ident(&db, "o"),
        mode: Mode::Output,
        typ,
    });
    let net_port_out = db.intern_net(Net::Port(port_out));

    let comp_a = db.intern_component(
        Component::new(ident(&db, "a"))
            .with_net(net_port_in)
            .with_net(net_port_out),
    );

    let inst_q = db.intern_instance(Instance::new(ident(&db, "q"), comp_a));
    let inst_r = db.intern_instance(Instance::new(ident(&db, "r"), comp_a));

    let q_i = db.intern_net(Net::InstancePort(InstancePort::new(inst_q, port_in)));
    let q_o = db.intern_net(Net::InstancePort(InstancePort::new(inst_q, port_out)));
    let r_i = db.intern_net(Net::InstancePort(InstancePort::new(inst_r, port_in)));
    let r_o = db.intern_net(Net::InstancePort(InstancePort::new(inst_r, port_out)));

    let top = db.intern_component(
        Component::new(ident(&db, "top"))
            .with_instance(inst_q)
            .with_instance(inst_r)
            .with_net(q_i)
            .with_net(r_i)
            .with_net(q_o)
            .with_net(r_o)
            .with_connection(Connection::new(q_o, r_i))
            .with_connection(Connection::new(q_i, r_o)),
    );

    let module = db.intern_module(Module {
        identifier: ident(&db, "p"),
        components: vec![comp_a, top],
        types: vec![typ],
    });

    let project = Project {
        modules: vec![module],
    };

    db.set_project(Arc::new(project));

    db
}
