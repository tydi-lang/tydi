#[salsa::database(tydi_hir::InternHirDatabase, tydi_intern::InternSupportDatabase)]
#[derive(Default)]
pub struct Database {
    storage: salsa::Storage<Database>,
}

impl salsa::Database for Database {}

#[cfg(test)]
mod tests {
    use super::Database;
    use tydi_hir::{Identifier, InternHir, Package, Type, TypeRefData};
    use tydi_intern::{InternSupport, IntoRefData};

    const IDENT: &str = "test";

    #[test]
    fn intern_support() {
        let db = Database::default();

        let id = db.intern_string(IDENT.to_string());
        assert_eq!(db.lookup_intern_string(id), IDENT.to_string());
    }

    #[test]
    fn intern_hir() {
        let db = Database::default();

        let string_id = db.intern_string(IDENT.to_string());
        let package_ref_data = Package {
            identifier: Identifier(String::from(IDENT)),
        }
        .into_ref_data(&db);

        let identifier_ref_data = Identifier(String::from(IDENT)).into_ref_data(&db);
        assert_eq!(package_ref_data.identifier, identifier_ref_data);
        assert_eq!(identifier_ref_data._0, string_id);

        let package_id = db.intern_package(package_ref_data.clone());
        assert_eq!(db.lookup_intern_package(package_id), package_ref_data);

        let type_ref_data = Type::Path {
            segments: vec![Identifier(String::from(IDENT))],
        }
        .into_ref_data(&db);
        assert_eq!(
            type_ref_data,
            TypeRefData::Path {
                segments: vec![identifier_ref_data],
            }
        );
    }
}
