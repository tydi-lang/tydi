#[tydi_macros::ir]
mod hir {
    pub struct Identifier(pub String);

    pub struct Package {
        pub identifier: Identifier,
    }
}

pub use hir::*;
