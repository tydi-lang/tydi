use proc_macro::TokenStream;

#[path = "ir.rs"]
mod ir_impl;

/// Attribute macro for generation of IR boilerplate. To be used on modules
/// with struct and enum items (todo).
#[proc_macro_attribute]
pub fn ir(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ir_impl::gen(item)
}
