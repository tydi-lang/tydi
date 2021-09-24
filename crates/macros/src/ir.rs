use heck::{SnakeCase, TitleCase};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse, punctuated::Punctuated, token::Comma, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    Index, Item, ItemEnum, ItemMod, ItemStruct, Type, Variant,
};

fn item_ident(item: &Item) -> &Ident {
    match item {
        Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. }) => ident,
        _ => unimplemented!(),
    }
}

fn intern_id_ident(ident: &Ident) -> Ident {
    format_ident!("{}Id", ident)
}

fn ref_data_ident(ident: &Ident) -> Ident {
    format_ident!("{}RefData", ident)
}

fn intern_fn_ident(ident: &Ident) -> Ident {
    format_ident!("intern_{}", ident.to_string().to_snake_case())
}

fn fields_iter(fields: &Fields) -> impl Iterator<Item = Field> {
    (match fields.clone() {
        Fields::Unit => Punctuated::<Field, Comma>::new(),
        Fields::Named(FieldsNamed { named: fields, .. })
        | Fields::Unnamed(FieldsUnnamed {
            unnamed: fields, ..
        }) => fields,
    })
    .into_iter()
}

fn fields_ident_iter(fields: &Fields) -> impl Iterator<Item = Ident> {
    fields_iter(fields)
        .enumerate()
        .map(|(idx, Field { ident, .. })| ident.unwrap_or_else(|| format_ident!("_{}", idx)))
}

fn fields_index_iter(fields: &Fields) -> impl Iterator<Item = TokenStream2> {
    fields_iter(fields)
        .enumerate()
        .map(|(idx, Field { ident, .. })| match ident {
            Some(ident) => quote!(#ident),
            None => {
                let idx = Index::from(idx);
                quote!(#idx)
            }
        })
}

fn fields_types_iter(fields: &Fields) -> impl Iterator<Item = Type> {
    fields_iter(fields).map(|Field { ty, .. }| ty)
}

type Variants = Punctuated<Variant, Comma>;

fn variant_ref_data_iter(variants: &Variants) -> impl Iterator<Item = TokenStream2> + '_ {
    variants
        .into_iter()
        .map(|Variant { ident, fields, .. }| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let ty = unnamed.into_iter().map(|Field { ty, .. }| ty);
                quote! {
                    #ident(
                        #(
                            #ty,
                        )*
                    )
                }
            }
            Fields::Unit => quote!(#ident),
            Fields::Named(FieldsNamed { named, .. }) => {
                let field = named
                    .iter()
                    .map(|Field { ident, .. }| ident.as_ref().unwrap());
                let ty = named.iter().map(|Field { ty, .. }| ty);
                quote!(
                    #ident {
                        #(
                            #field: <#ty as ::tydi_intern::IntoRefData>::RefData,
                        )*
                    }
                )
            }
        })
}

fn variant_into_ref_data_iter(variants: &Variants) -> impl Iterator<Item = TokenStream2> + '_ {
    variants
        .into_iter()
        .map(|Variant { ident, fields, .. }| match fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let idx = (0..unnamed.iter().count())
                    .into_iter()
                    .map(|idx| format_ident!("_{}", idx))
                    .collect::<Vec<_>>();
                quote! {
                    #ident(
                        #(
                            #idx,
                        )*
                    ) => {
                        Self::RefData::#ident(
                            #(
                                #idx.into_ref_data(db),
                            )*
                        )
                    }
                }
            }
            Fields::Unit => quote!(#ident => Self::RefData::#ident),
            Fields::Named(FieldsNamed { named, .. }) => {
                let fields = named
                    .into_iter()
                    .map(|Field { ident, .. }| ident.as_ref().unwrap())
                    .collect::<Vec<_>>();
                quote! {
                    #ident {
                        #(
                            #fields,
                        )*
                    } => {
                        Self::RefData::#ident {
                            #(
                                #fields: #fields.into_ref_data(db)
                            )*
                        }
                    }
                }
            }
        })
}

pub(super) fn gen(item: TokenStream) -> TokenStream {
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
            // Skip over everything that is not a struct or enum
            .filter(|item| matches!(item, Item::Struct(_) | Item::Enum(_)));

        // Generate the intern trait.
        let title_case = ident.to_string().to_title_case();
        let intern_trait = format_ident!("Intern{}", title_case);
        let intern_storage = format_ident!("Intern{}Database", title_case);
        let intern_id = items
            .clone()
            .map(|item| intern_id_ident(item_ident(&item)))
            .collect::<Vec<_>>();
        let intern_ref_data = items
            .clone()
            .map(|item| ref_data_ident(item_ident(&item)))
            .collect::<Vec<_>>();
        let intern_fn = items.clone().map(|item| intern_fn_ident(item_ident(&item)));
        tokens.extend(quote!(
            #[automatically_derived]
            #[::salsa::query_group(#intern_storage)]
            pub trait #intern_trait: ::tydi_intern::InternSupport {
                #(
                    #[::salsa::interned]
                    fn #intern_fn(&self, data: #intern_ref_data) -> #intern_id;
                )*
            }
        ));

        // Generate Id struct.
        tokens.extend(items.clone().map(|item| {
            let ident = item_ident(&item);
            let id = intern_id_ident(ident);

            quote!(
                #[automatically_derived]
                ::tydi_intern::intern_id!(#ident, #id);
            )
        }));

        // Generate RefData struct and IntoRefData impl.
        tokens.extend(items.clone().map(|item| {
            match item {
                Item::Struct(ItemStruct {  ident, fields, .. }) => {
                    let id = intern_id_ident(&ident);
                    let ref_data = ref_data_ident(&ident);
                    let field_ident = fields_ident_iter(&fields).collect::<Vec<_>>();
                    let field_index = fields_index_iter(&fields);
                    let field_types = fields_types_iter(&fields);

                    quote!(
                        #[automatically_derived]
                        #[derive(Clone, Debug, Hash, PartialEq, Eq)]
                        pub struct #ref_data {
                            #(
                                pub #field_ident: <#field_types as ::tydi_intern::IntoRefData>::RefData,
                            )*
                        }

                        #[automatically_derived]
                        impl ::tydi_intern::IntoRefData for #ident {
                            type Id = #id;
                            type RefData = #ref_data;
                            fn into_ref_data(self, db: &dyn ::tydi_intern::InternSupport) -> Self::RefData {
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
                },
                Item::Enum(ItemEnum { ident, variants, .. }) => {
                    let id = intern_id_ident(&ident);
                    let ref_data = ref_data_ident(&ident);
                    let variant_ref_data = variant_ref_data_iter(&variants);
                    let variant_into_ref_data = variant_into_ref_data_iter(&variants);

                    quote!(
                        #[automatically_derived]
                        #[derive(Clone, Debug, Hash, PartialEq, Eq)]
                        pub enum #ref_data {
                            #(
                                #variant_ref_data,
                            )*
                        }

                        #[automatically_derived]
                        impl ::tydi_intern::IntoRefData for #ident {
                            type Id = #id;
                            type RefData = #ref_data;
                            fn into_ref_data(self, db: &dyn ::tydi_intern::InternSupport) -> Self::RefData {
                                match self {
                                    #(
                                        Self::#variant_into_ref_data,
                                    )*
                                }
                            }
                        }
                    )
                }
                _ => unreachable!()
            }}));

        tokens = quote!(
            mod gen {
                use super::*;

                #tokens
            }
            pub use gen::#intern_trait;
            pub use gen::#intern_storage;
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
