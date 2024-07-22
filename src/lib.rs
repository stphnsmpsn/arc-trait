extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, TraitItemFn};

#[proc_macro_attribute]
pub fn arc_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the trait
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;

    // Create the trait definition token to keep it intact
    let trait_def = quote! {
        #input
    };

    // Collect all methods in the trait
    let methods: Vec<TraitItemFn> = input.items.iter().filter_map(|item| {
        if let TraitItem::Fn(method) = item {
            Some(method.clone())
        } else {
            None
        }
    }).collect();

    // Generate implementations for Arc<T>
    let impls = methods.iter().map(|method| {
        let name = &method.sig.ident;
        let inputs = &method.sig.inputs;
        let output = &method.sig.output;
        let generics = &method.sig.generics;
        let where_clause = &method.sig.generics.where_clause;
        let attrs = &method.attrs;
        let is_async = method.sig.asyncness.is_some();

        let call_args = inputs.iter().skip(1).map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                quote! { #pat }
            } else {
                quote! {}
            }
        });

        if is_async {
            quote! {
                #(#attrs)*
                async fn #name #generics (#inputs) #output #where_clause {
                    self.as_ref().#name(#(#call_args),*).await
                }
            }
        } else {
            quote! {
                #(#attrs)*
                fn #name #generics (#inputs) #output #where_clause {
                    self.as_ref().#name(#(#call_args),*)
                }
            }
        }
    });

    let expanded = if methods.iter().any(|method| method.sig.asyncness.is_some()) {
        quote! {
            #trait_def

            #[async_trait::async_trait]
            impl<T: #trait_name + Send + Sync> #trait_name for std::sync::Arc<T> {
                #(#impls)*
            }
        }
    } else {
        quote! {
            #trait_def

            impl<T: #trait_name> #trait_name for std::sync::Arc<T> {
                #(#impls)*
            }
        }
    };

    // Return the generated implementation
    TokenStream::from(expanded)
}