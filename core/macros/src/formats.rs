use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_serialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let fields = match input.data {
    syn::Data::Struct(ref data) => match data.fields {
      syn::Fields::Named(ref fields) => fields.named.iter().map(|f| &f.ident),
      _ => todo!(),
    },
    _ => todo!(),
  };

  let expanded = quote! {
    impl Serialize for #ident {
      fn serialize(&self) -> Chunk {
        let mut fields = FastHashMap::default();

        #(
          fields.insert(
            stringify!(#fields).to_string(),
            self.#fields.serialize()
          );
        )*

        Chunk::Map(fields)
      }
    }
  };

  expanded.into()
}

pub fn impl_deserialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let _fields = match input.data {
    syn::Data::Struct(ref data) => match data.fields {
      syn::Fields::Named(ref fields) => fields.named.iter().map(|f| &f.ident),
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  };

  let expanded = quote! {
    impl Deserialize for #ident {
      fn deserialize(chunk: &Chunk) -> Self {
        match chunk {
          Chunk::Map(fields) => {
            let mut instance = Self::default();

            #(
              if let Some(chunk) = fields.get(stringify!(#_fields)) {
                instance.#_fields = Deserialize::deserialize(chunk);
              }
            )*

            instance
          }
          _ => todo!(),
        }
      }
    }
  };

  expanded.into()
}
