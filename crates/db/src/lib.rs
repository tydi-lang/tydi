#[salsa::database(tydi_hir::HirStorage)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Database>,
}

impl salsa::Database for Database {}
