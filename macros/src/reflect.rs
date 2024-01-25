use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn impl_reflect(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let output = match input.data {
    // expand structs into reflectable types
    syn::Data::Struct(ref struct_info) => {
      let fields = match &struct_info.fields {
        syn::Fields::Named(fields) => fields.named.iter(),
        syn::Fields::Unnamed(fields) => fields.unnamed.iter(),
        _ => panic!("Only structs are supported"),
      }
      .map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {
          name: stringify!(#field_name),
          kind: stringify!(#field_type),
          offset: 0, /* std::mem::offset_of!(#ident, #field_name) */
        }
      });

      let length = fields.len();

      quote! {
        impl StructType for #ident {
          #[inline]
          fn fields(&self) -> &[FieldInfo] {
            static FIELDS: [FieldInfo; #length] = [
              #(FieldInfo { #fields },)*
            ];

            &FIELDS
          }
        }
      }
    }
    syn::Data::Enum(_enum_info) => {
      todo!();
    }
    syn::Data::Union(_union_info) => {
      todo!();
    }
  };

  output.into()
}
