use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

pub fn impl_vertex_trait(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let descriptors = parse_struct(&input.data);
  let name = &input.ident;

  let expanded = quote! {
    impl Vertex for #name {
      const DESCRIPTORS: &'static [VertexDescriptor] = &[
        #(#descriptors),*
      ];
    }
  };

  expanded.into()
}

/// Parses the struct and returns a list of vertex descriptors.
fn parse_struct(data: &Data) -> Vec<proc_macro2::TokenStream> {
  match data {
    Data::Struct(ref data) => match data.fields {
      Fields::Named(ref fields) => fields
        .named
        .iter()
        .map(|field| {
          let (count, kind, normalize) = parse_vertex_attributes(&field.attrs);

          quote_spanned! { field.span() =>
            VertexDescriptor {
              count: #count,
              kind: VertexKind::#kind,
              should_normalize: #normalize,
            }
          }
        })
        .collect(),
      Fields::Unnamed(_) => panic!("`#[derive(Vertex)]` does not support tuple structs"),
      Fields::Unit => panic!("`#[derive(Vertex)]` does not support unit structs"),
    },
    Data::Enum(_) => panic!("`#[derive(Vertex)]` does not support enums"),
    Data::Union(_) => panic!("`#[derive(Vertex)]` does not support unions"),
  }
}

/// Parses the `#[vertex]` attribute on a field.
fn parse_vertex_attributes(attributes: &Vec<Attribute>) -> (usize, proc_macro2::TokenStream, bool) {
  let mut count = None;
  let mut kind = None;
  let mut normalize = false;

  for attribute in attributes {
    if let Ok(meta) = attribute.parse_meta() {
      if meta.path().is_ident("vertex") {
        if let Meta::List(list) = meta {
          // extract count, kind and normalize from the attribute based on order
          let entries = list.nested.iter().collect::<Vec<_>>();

          let count_entry = entries.first();
          let kind_entry = entries.get(1);
          let normalize_entry = entries.get(2);

          if let Some(NestedMeta::Lit(Lit::Int(value))) = count_entry {
            count = Some(value.base10_parse::<usize>().unwrap());
          } else {
            panic!("`#[vertex]` attribute requires a count");
          }

          if let Some(NestedMeta::Meta(Meta::Path(path))) = kind_entry {
            kind = Some(quote! { #path });
          } else {
            panic!("`#[vertex]` attribute requires a kind");
          }

          if let Some(NestedMeta::Meta(Meta::Path(path))) = normalize_entry {
            normalize = path.is_ident("normalize");
          }
        }
      }
    }
  }

  match (count, kind) {
    (Some(count), Some(kind)) => (count, kind, normalize),
    _ => panic!("`#[vertex]` attribute is missing required fields"),
  }
}
