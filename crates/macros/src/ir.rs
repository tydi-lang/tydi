use heck::{SnakeCase, TitleCase};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse, punctuated::Punctuated, token::Comma, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Index, Item, ItemMod, ItemStruct, Type,
};

// Capture the required information for code generation.
enum InternItem {
    Struct { ident: Ident, fields: Fields },
    // todo(mb)
    // Enum {},
}

impl InternItem {
    /// Returns ident from struct or enum.
    fn ident(&self) -> &Ident {
        match self {
            InternItem::Struct { ident, .. } => ident,
            // _ => todo!(),
        }
    }

    /// Returns ident for ID struct.
    fn intern_id(&self) -> Ident {
        format_ident!("{}Id", self.ident())
    }

    /// Returns ident for RefData struct.
    fn ref_data(&self) -> Ident {
        format_ident!("{}RefData", self.ident())
    }

    /// Returns ident for intern trait fn.
    fn intern_fn(&self) -> Ident {
        format_ident!("intern_{}", self.ident().to_string().to_snake_case())
    }

    /// Return an iterator over the fields of the struct.
    fn fields(&self) -> impl Iterator<Item = Field> {
        match self {
            InternItem::Struct { fields, .. } => match fields.clone() {
                Fields::Unit => Punctuated::<Field, Comma>::new().into_iter(),
                Fields::Named(FieldsNamed { named: fields, .. })
                | Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }) => fields.into_iter(),
            },
            // _ => todo!(),
        }
    }

    /// Return an iterator over the idents of the fields of the struct.
    fn field_ident(&self) -> impl Iterator<Item = Ident> {
        self.fields()
            .enumerate()
            .map(|(idx, Field { ident, .. })| ident.unwrap_or_else(|| format_ident!("_{}", idx)))
    }

    /// Returns an iterator over idents that can be used to index fields.
    fn field_index(&self) -> impl Iterator<Item = TokenStream2> {
        self.fields()
            .enumerate()
            .map(|(idx, Field { ident, .. })| match ident {
                Some(ident) => quote!(#ident),
                None => {
                    let idx = Index::from(idx);
                    quote!(#idx)
                }
            })
    }

    /// Returns an iterator over the types of the fields of the struct.
    fn field_types(&self) -> impl Iterator<Item = Type> {
        self.fields().map(|Field { ty, .. }| ty)
    }
}

pub(super) fn gen(item: TokenStream) -> TokenStream {
    // Generated code depends on the `salsa` and `tydi_intern` crates.
    let salsa = format_ident!("salsa");
    let tydi_intern = format_ident!("tydi_intern");

    // Keep copy of original input.
    let input: TokenStream2 = item.clone().into();

    // Inner gen module tokenstream.
    let mut tokens = quote!();

    // Make sure the attribute is placed on a `mod` item.
    if let Ok(Item::Mod(ItemMod { ident, content, .. })) = parse(item) {
        // Iterate over all the items in the module.
        let items = content
            .expect("can't be used on an empty mod")
            .1
            .into_iter()
            // Skip over everything that is not a struct (todo(mb) or enum)
            .filter(|item| matches!(item, Item::Struct(_)))
            .map(|item| match item {
                Item::Struct(ItemStruct { ident, fields, .. }) => {
                    InternItem::Struct { ident, fields }
                }
                _ => unreachable!(),
            });

        // Generate the intern trait.
        let title_case = ident.to_string().to_title_case();
        let intern_trait = format_ident!("Intern{}", title_case);
        let intern_storage = format_ident!("Intern{}Database", title_case);
        let intern_id = items
            .clone()
            .map(|item| item.intern_id())
            .collect::<Vec<_>>();
        let intern_ref_data = items
            .clone()
            .map(|item| item.ref_data())
            .collect::<Vec<_>>();
        let intern_fn = items.clone().map(|item| item.intern_fn());
        tokens.extend(quote!(
            #[automatically_derived]
            #[#salsa::query_group(#intern_storage)]
            pub trait #intern_trait: #tydi_intern::InternSupport {
                #(
                    #[#salsa::interned]
                    fn #intern_fn(&self, data: #intern_ref_data) -> #intern_id;
                )*
            }
        ));

        // Generate id and ref data structures.
        tokens.extend(items.clone().map(|item| {
            let ident = item.ident();
            let id = item.intern_id();
            let ref_data = item.ref_data();
            let field_ident = item.field_ident().collect::<Vec<_>>();
            let field_index = item.field_index();
            let field_types = item.field_types();

            // Generate the id and ref data structures and impl ToRefData.
            quote!(
                #[automatically_derived]
                #tydi_intern::intern_id!(#ident, #id);

                #[automatically_derived]
                #[derive(Clone, Debug, Hash, PartialEq, Eq)]
                pub struct #ref_data {
                    #(
                        pub #field_ident: <#field_types as #tydi_intern::IntoRefData>::RefData,
                    )*
                }

                #[automatically_derived]
                impl #tydi_intern::IntoRefData for #ident {
                    type Id = #id;
                    type RefData = #ref_data;
                    fn into_ref_data(self, db: &dyn #tydi_intern::InternSupport) -> Self::RefData {
                        #(
                            let #field_ident = self.#field_index.into_ref_data(db);
                        )*
                        Self::RefData {
                            #(
                                #field_ident,
                            )*
                        }
                    }
                }
            )
        }));
        tokens = quote!(
            mod gen {
                use super::*;

                #tokens
            }
            pub use gen::#intern_trait;
            #(
                pub use gen::#intern_id;
                pub use gen::#intern_ref_data;
            )*
        );
    } else {
        panic!("must be  used on `mod` item")
    };

    quote!(
        #input
        #tokens
    )
    .into()
}
