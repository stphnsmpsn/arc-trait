extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem};

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
    let methods: Vec<_> = input.items.iter().filter_map(|item| {
        if let TraitItem::Fn(method) = item {
            Some(method)
        } else {
            None
        }
    }).collect();

    // Generate implementations for Arc<T>
    let impls = methods.iter().map(|method| {

        let name = &method.sig.ident;
        let inputs = &method.sig.inputs;
        let output = &method.sig.output;

        let call_args = inputs.iter().skip(1).map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                let pat = &pat_type.pat;
                quote! { #pat }
            } else {
                quote! {}
            }
        });

        quote! {
            fn #name (#inputs) #output {
                self.as_ref().#name(#(#call_args),*)
            }
        }
    });

    let expanded = quote! {
        #trait_def

        impl<T: #trait_name> #trait_name for std::sync::Arc<T> {
            #(#impls)*
        }
    };

    // Return the generated implementation
    TokenStream::from(expanded)
}