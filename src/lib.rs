extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, DataEnum};
use syn::punctuated::Punctuated;

#[proc_macro_derive(Values)]
pub fn values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let dataEnum = match input.data {
        syn::Data::Enum(data_enum) => {
            data_enum
        }
        _ =>{
            panic!("{} must be an enum", name)
        }
    };

    let variants = dataEnum.variants;
    let variant_count = variants.len();
    let variants = variants.iter();

    let expanded = quote! {

        impl #name {
            pub fn values() -> [#name; #variant_count]{
                [#(
                    #name::#variants,
                )*]
            }
        }
    };
    TokenStream::from(expanded)

}

