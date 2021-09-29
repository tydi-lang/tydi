use std::sync::Arc;
use tydi_intern::Id;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Root {
    pub modules: Vec<Id<Module>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Module {
    pub identifier: Id<Identifier>,
    pub statements: Vec<Id<Statement>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    TypeDefinition(TypeDefinition),
    Other,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeDefinition {
    pub identifier: Id<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

#[salsa::query_group(HirStorage)]
pub trait Hir {
    // Temporary input.
    #[salsa::input]
    fn root(&self) -> Arc<Root>;

    #[salsa::interned]
    fn intern_module(&self, module: Module) -> Id<Module>;
    #[salsa::interned]
    fn intern_statement(&self, statement: Statement) -> Id<Statement>;
    #[salsa::interned]
    fn intern_identifier(&self, identifier: Identifier) -> Id<Identifier>;

    fn modules(&self) -> Arc<Vec<Id<Module>>>;
    fn module_statements(&self, module: Id<Module>) -> Arc<Vec<Id<Statement>>>;
    fn module_type_definitions(&self, module: Id<Module>) -> Arc<Vec<TypeDefinition>>;

    fn statements(&self) -> Arc<Vec<Id<Statement>>>;
    fn type_definitions(&self) -> Arc<Vec<TypeDefinition>>;
}

fn modules(db: &dyn Hir) -> Arc<Vec<Id<Module>>> {
    Arc::new(db.root().modules.clone())
}

fn module_statements(db: &dyn Hir, module: Id<Module>) -> Arc<Vec<Id<Statement>>> {
    Arc::new(db.lookup_intern_module(module).statements)
}

fn module_type_definitions(db: &dyn Hir, module: Id<Module>) -> Arc<Vec<TypeDefinition>> {
    Arc::new(
        db.module_statements(module)
            .iter()
            .map(|&id| db.lookup_intern_statement(id))
            .filter_map(|statement| match statement {
                Statement::TypeDefinition(type_definition) => Some(type_definition),
                _ => None,
            })
            .collect(),
    )
}

fn statements(db: &dyn Hir) -> Arc<Vec<Id<Statement>>> {
    Arc::new(
        db.modules()
            .iter()
            .flat_map(|&id| db.module_statements(id).iter().copied().collect::<Vec<_>>())
            .collect(),
    )
}

fn type_definitions(db: &dyn Hir) -> Arc<Vec<TypeDefinition>> {
    Arc::new(
        db.modules()
            .iter()
            .flat_map(|&id| {
                db.module_type_definitions(id)
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}
