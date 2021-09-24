#[tydi_macros::ir]
mod hir {
    pub struct Identifier(pub String);

    pub struct Package {
        pub identifier: Identifier,
    }

    pub enum Type {
        Bits(usize),
        Stream,
        Path { segments: Vec<Identifier> },
    }
}

pub use hir::*;
