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

    // Extract generics
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate layout information
    let layout = layout(&input.data);

    let expanded = match layout {
        Either::Left((ts_get_layout_callback, ts_get_layout_type_callback)) => {
            quote! {
                impl #impl_generics layout_trait::GetLayout for #name #ty_generics #where_clause {
                    fn get_layout<F: FnMut(&self,usize, usize)>(&self, f: &mut F) {
                        #ts_get_layout_callback
                    }
                }

                impl #impl_generics layout_trait::GetLayoutType for #name #ty_generics #where_clause {
                    fn get_layout_type<F: FnMut(usize, usize)>( f: &mut F) {
                        #ts_get_layout_type_callback
                    }
                }
            }
        }
        Either::Right(ts_callback) => {
            quote! {
                impl #impl_generics layout_trait::GetLayoutType for #name #ty_generics #where_clause  {
                    fn get_layout_type<F: FnMut(usize, usize)>( f: &mut F) {
                        #ts_callback
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
                    let recurse_callback = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        match name {
                            Some(inner_name) => {
                                quote_spanned! {f.span()=>
                                    self.#inner_name.get_layout(f)
                                }
                            },
                            None =>  quote_spanned! {f.span()=> compile_error!("Cannot derive layout for structs with unnamed members")}
                        }
                    });
                    let recurse_type_callback = fields.named.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! {f.span()=>
                            <#name>::get_layout_type(f)
                        }
                    });

                    Either::Left((
                        quote_spanned! {data.struct_token.span() =>
                            #(#recurse_callback;)*
                        },
                        quote_spanned! {data.struct_token.span() =>
                            #(#recurse_type_callback;)*
                        },
                    ))
                }
                Fields::Unnamed(ref fields) => {
                    let recurse_callback = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            self.#index.get_layout(f)
                        }
                    });
                    let recurse_type_callback = fields.unnamed.iter().map(|f| {
                        let name = &f.ty;
                        quote_spanned! {f.span()=>
                            #name::get_layout_type(f)
                        }
                    });
                    Either::Left((
                        quote_spanned! {fields.span() =>
                           #(#recurse_callback;)*
                        },
                        quote_spanned! {fields.span() =>
                            #(#recurse_type_callback;)*
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
            let mut rec_callback: Vec<TokenStream> = vec![];
            for v in data.variants.iter() {
                for f in v.fields.iter() {
                    let ty = &f.ty;

                    match ty {
                        Type::Path(path) => {
                            rec_callback.push(quote_spanned! {f.span()=>
                                #path::get_layout_type(f);
                            });
                        }
                        _ => {}
                    }
                }
            }

            Either::Right(quote_spanned! { data.enum_token.span() =>
                #(#rec_callback;)*
            })
        }
        Data::Union(_) => unimplemented!(),
    }
}
