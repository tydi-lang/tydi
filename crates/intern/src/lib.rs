/// Macro to generate new type wrappers for Salsa's intern ID.
#[macro_export]
macro_rules! intern_id {
    ($item:ident, $id:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $id(salsa::InternId);

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl salsa::InternKey for $id {
            fn from_intern_id(id: salsa::InternId) -> Self {
                Self(id)
            }
            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    };
}

intern_id!(String, StringId);

mod query {
    use super::*;

    /// Super trait for intern traits with intern support for primitive types.
    #[salsa::query_group(InternSupportDatabase)]
    pub trait InternSupport {
        #[salsa::interned]
        fn intern_string(&self, string: String) -> StringId;
    }
}

// Export the query group trait and storage struct.
pub use query::{InternSupport, InternSupportDatabase};

/// Transforms owned data types to referenced data types.
pub trait IntoRefData {
    /// Identifier type used when interning this type.
    type Id;
    /// Reference data type for this type. This replaces owned values with
    /// reference data types to interned values.
    type RefData;

    /// Interns self in the database and returns an instance of the reference
    /// data type.
    fn into_ref_data(self, db: &dyn InternSupport) -> Self::RefData;
}

// Implement IntoRefData without interning i.e. Self is Id and RefData.
macro_rules! into_ref_data_self {
    ($($ident:ident),*) => {
        $(
            impl IntoRefData for $ident {
                type Id = Self;
                type RefData = Self;
                fn into_ref_data(self, _db: &dyn InternSupport) -> Self::RefData {
                    self
                }
            }
        )*
    };
}

// These copy types are not interned and stored in reference data types.
into_ref_data_self!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl IntoRefData for String {
    type Id = StringId;
    type RefData = StringId;
    fn into_ref_data(self, db: &dyn InternSupport) -> Self::RefData {
        // Insert the String in the intern database.
        db.intern_string(self)
    }
}

impl<T> IntoRefData for Vec<T>
where
    T: IntoRefData,
{
    type Id = <T as IntoRefData>::Id;
    type RefData = Vec<<T as IntoRefData>::RefData>;

    fn into_ref_data(self, db: &dyn InternSupport) -> Self::RefData {
        // Return new vec with the reference data types of all the items in the
        // vec.
        self.into_iter()
            .map(|item| item.into_ref_data(db))
            .collect()
    }
}

impl<T> IntoRefData for Option<T>
where
    T: IntoRefData,
{
    type Id = <T as IntoRefData>::Id;
    type RefData = Option<<T as IntoRefData>::RefData>;

    fn into_ref_data(self, db: &dyn InternSupport) -> Self::RefData {
        // Return new option with the reference data type of the inner value of
        // the Option.
        self.map(|item| item.into_ref_data(db))
    }
}

impl<T> IntoRefData for Box<T>
where
    T: IntoRefData,
{
    type Id = <T as IntoRefData>::Id;
    type RefData = Box<<T as IntoRefData>::RefData>;

    fn into_ref_data(self, db: &dyn InternSupport) -> Self::RefData {
        // Return new box with reference data type of the inner value of the
        // Box.
        Box::new((*self).into_ref_data(db))
    }
}
