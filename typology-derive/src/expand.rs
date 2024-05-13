use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
	parse::{Parse, ParseStream},
	Data, DeriveInput, Error, Fields, Ident, LitInt, Result, Token,
};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
	let vis = input.vis;
	let tymod = format_ident!("{}Typology", input.ident);

	let mut derived_types = Vec::new();

	let mut push_type = |fields: Fields, variant: Option<Ident>| {
		for (idx, field) in fields.into_iter().enumerate() {
			let ident = match (field.ident, &variant) {
				(Some(ident), None) => format_ident!("__type_{ident}"),
				(None, None) => format_ident!("__type_{idx}"),
				(Some(ident), Some(variant)) => format_ident!("__type_{variant}_{ident}"),
				(None, Some(variant)) => format_ident!("__type_{variant}_{idx}"),
			};

			let ty = field.ty;

			derived_types.push(quote! {
				pub type #ident = #ty;
			});
		}
	};

	match input.data {
		Data::Struct(data) => {
			push_type(data.fields, None);
		}
		Data::Enum(data) => {
			for variant in data.variants {
				push_type(variant.fields, Some(variant.ident));
			}
		}
		Data::Union(_) => Err(Error::new(Span::call_site(), "Unions not supported"))?,
	}

	let output = quote! {
		#vis mod #tymod {
			#(#derived_types)*
		}
	};

	Ok(output)
}

pub fn expand_type_of(input: TypeofInput) -> TokenStream {
	let tymod = format_ident!("{}Typology", input.target);

	let ident = match input.of {
		Of::Variant(variant, field) => format_ident!("__type_{variant}{}", field.into_ident()),
		Of::Field(field) => format_ident!("__type{}", field.into_ident()),
	};

	quote!(#tymod::#ident)
}

pub enum FieldName {
	Ident(Ident),
	Lit(LitInt),
}

impl FieldName {
	fn into_ident(self) -> Ident {
		match self {
			FieldName::Ident(ident) => format_ident!("_{ident}"),
			FieldName::Lit(ident) => format_ident!("_{ident}"),
		}
	}
}

impl Parse for FieldName {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(LitInt) {
			Ok(Self::Lit(input.parse()?))
		} else if input.peek(Ident) {
			Ok(Self::Ident(input.parse()?))
		} else {
			Err(input.error("Expected field name"))
		}
	}
}

pub enum Of {
	Variant(Ident, FieldName),
	Field(FieldName),
}

impl Parse for Of {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek2(Token![::]) {
			let ident: Ident = input.parse()?;
			input.parse::<Token![::]>()?;
			let name: FieldName = input.parse()?;
			Ok(Self::Variant(ident, name))
		} else {
			Ok(Self::Field(input.parse()?))
		}
	}
}

pub struct TypeofInput {
	target: Ident,
	of: Of,
}

impl Parse for TypeofInput {
	fn parse(input: ParseStream) -> Result<Self> {
		let target: Ident = input.parse()?;
		input.parse::<Token![::]>()?;
		Ok(Self { target, of: input.parse()? })
	}
}
