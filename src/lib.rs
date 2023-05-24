use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

// Derive macro loosely based on the `syn` `HeapSize` example

#[proc_macro_derive(Layout)]
pub fn derive_layout(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Generate layout information
    let layout = layout(&input.data);

    let expanded = quote! {
        impl layout_trait::GetLayout for #name  {
            fn get_layout<const N: usize>(&self, layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>) {
                #layout
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Generate layout information
fn layout(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            self.#name.get_layout(layout)
                        }
                    });
                    quote! {
                        #(#recurse;)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            self.#index.get_layout(layout)
                        }
                    });
                    quote! {
                        #(#recurse;)*
                    }
                }
                Fields::Unit => {
                    // Unit structs have no layout

                    unimplemented!()
                }
            }
        }
        Data::Enum(ref data) => {
            let mut rec: Vec<TokenStream> = vec![];
            data.variants.iter().clone().for_each(|f| {
                // let name = &f.ident;
                let recurse = &f.fields.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {f.span()=>
                        self.#index.get_layout(layout)
                    }
                });
                let recurse = recurse.clone();
                let ts = quote! {
                    #(#recurse;)*
                }
                .clone();
                rec.push(ts);
            });
            quote! {
             #(#rec;)*
            }
            // quote! {}
        }
        Data::Union(_) => unimplemented!(),
    }
}
