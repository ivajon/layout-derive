use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index, Type};

use either::Either;

// Derive macro loosely based on the `syn` `HeapSize` example
#[proc_macro_derive(Layout)]
pub fn derive_layout(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Generate layout information
    let layout = layout(&input.data);

    let expanded = match layout {
        Either::Left((ts_get_layout, ts_get_layout_type)) => {
            quote! {
                impl layout_trait::GetLayout for #name  {
                    fn get_layout<const N: usize>(&self, layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>) {
                        #ts_get_layout
                    }
                }

                impl layout_trait::GetLayoutType for #name  {
                    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>) {
                        #ts_get_layout_type
                    }
                }
            }
        }
        Either::Right(ts) => {
            quote! {
                impl layout_trait::GetLayoutType for #name  {
                    fn get_layout_type<const N: usize>(layout: &mut layout_trait::heapless::Vec<layout_trait::Layout, N>) {
                        #ts
                    }
                }
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Generate layout information
fn layout(data: &Data) -> Either<(TokenStream, TokenStream), TokenStream> {
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
                    let recurse_type = fields.named.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! {f.span()=>
                            <#name>::get_layout_type(layout)
                        }
                    });
                    Either::Left((
                        quote! {
                            #(#recurse;)*
                        },
                        quote! {
                            #(#recurse_type;)*
                        },
                    ))
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            self.#index.get_layout(layout)
                        }
                    });
                    let recurse_type = fields.unnamed.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! {f.span()=>
                            #name::get_layout_type(layout)
                        }
                    });
                    Either::Left((
                        quote! {
                           #(#recurse;)*
                        },
                        quote! {
                            #(#recurse_type;)*
                        },
                    ))
                }
                Fields::Unit => {
                    // Unit structs have no layout

                    unimplemented!()
                }
            }
        }
        Data::Enum(ref data) => {
            let mut rec: Vec<TokenStream> = vec![];
            for v in data.variants.iter() {
                for f in v.fields.iter() {
                    let ty = &f.ty;

                    match ty {
                        Type::Path(path) => {
                            rec.push(quote_spanned! {f.span()=>
                                #path::get_layout_type(layout);
                            });
                        }
                        _ => {}
                    }
                }
            }

            Either::Right(quote! {
                #(#rec;)*
            })
        }
        Data::Union(_) => unimplemented!(),
    }
}
