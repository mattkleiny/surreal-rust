use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

pub fn impl_vertex_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  // parses the given `struct`s fields into a series of `VertexDescriptor`s.
  let descriptors = match input.data {
    // discover struct definitions
    Data::Struct(ref data) => {
      match data.fields {
        // discover named fields (x: f32, y: f32, etc).
        Fields::Named(ref fields) => {
          fields.named.iter().map(|field| {
            // parse #[vertex(...)] attributes
            let (count, kind, normalize) = parse_vertex_attributes(&field.attrs);

            // build the descriptor
            quote_spanned! {field.span() =>
              VertexDescriptor { count: #count, kind: VertexKind::#kind, should_normalize: #normalize }
            }
          })
        }
        _ => panic!("Only named struct fields are supported"),
      }
    }
    _ => panic!("Only structs are supported"),
  };

  (quote! {
    impl Vertex for #name {
      const DESCRIPTORS: &'static [VertexDescriptor] = &[
        #(#descriptors),*
      ];
    }
  })
  .into()
}

fn parse_vertex_attributes(attributes: &Vec<Attribute>) -> (usize, proc_macro2::TokenStream, bool) {
  for attribute in attributes {
    if let Ok(meta) = attribute.parse_meta() {
      if meta.path().is_ident("vertex") {
        let count: Option<usize> = Some(2);
        let kind: Option<proc_macro2::TokenStream> = Some(quote!(F32));
        let normalize: Option<bool> = Some(false);

        match meta {
          Meta::List(ref list) => {
            for nested in list.nested.iter() {
              match nested {
                NestedMeta::Meta(ref meta) => match meta {
                  Meta::Path(_) => panic!("It was a path!"),
                  Meta::List(_) => panic!("It was a list!"),
                  Meta::NameValue(_) => panic!("It was a name value!"),
                },
                NestedMeta::Lit(ref literal) => match literal {
                  Lit::Str(_) => {}
                  Lit::ByteStr(_) => {}
                  Lit::Byte(_) => {}
                  Lit::Char(_) => {}
                  Lit::Int(_) => {}
                  Lit::Float(_) => {}
                  Lit::Bool(_) => {}
                  Lit::Verbatim(_) => {}
                },
              }
            }
          }
          _ => panic!("#[vertex] must be tuple of (count, kind, normalize)"),
        }

        return (
          count.expect("`#[vertex(count)]` is missing or invalid"),
          kind.expect("`#[vertex(kind)]` is missing or invalid"),
          normalize.expect("`#[vertex(normalize)]` is missing or invalid"),
        );
      }
    };
  }

  panic!("`#[vertex]` attribute is missing");
}
