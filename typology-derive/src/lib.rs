extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod expand;

#[proc_macro_derive(Typology, attributes(typology))]
pub fn typology_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input);
	expand::expand(input).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[proc_macro]
pub fn type_of(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input);
	expand::expand_type_of(input).into()
}
