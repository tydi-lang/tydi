#[salsa::database(tydi_hir::HirStorage)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Database>,
}

impl salsa::Database for Database {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ops::Deref, sync::Arc};
    use tydi_hir::{Hir, Identifier, Module, Root, Statement, TypeDefinition};

    #[test]
    fn basic() {
        let mut db = Database::default();

        let identifier = |name| db.intern_identifier(Identifier(String::from(name)));

        let module = |name, statements| {
            db.intern_module(Module {
                identifier: identifier(name),
                statements,
            })
        };
        let root = |modules| Root { modules };

        let type_def = |name| {
            db.intern_statement(Statement::TypeDefinition(TypeDefinition {
                identifier: identifier(name),
            }))
        };

        // Construct some input data.
        let foo = type_def("foo");
        let bar = type_def("bar");
        let foobar = type_def("foobar");
        let a = module("a", vec![foo, bar]);
        let b = module("b", vec![foobar]);
        let c = module("c", vec![]);

        db.set_root(Arc::new(root(vec![a, b])));
        assert_eq!(db.statements().deref(), &vec![foo, bar, foobar]);

        db.set_root(Arc::new(root(vec![a])));
        assert_eq!(db.statements().deref(), &vec![foo, bar]);

        db.set_root(Arc::new(root(vec![a, c])));
        assert_eq!(db.statements().deref(), &vec![foo, bar]);
    }
}
