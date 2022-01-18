extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    // Emit test garbage
    let tokens = quote! {
        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}

#[proc_macro]
pub fn my_macro_panics(_input: TokenStream) -> TokenStream {
    panic!("test")
}

#[proc_macro_derive(MyDerivePanics)]
pub fn my_derive_panics(_input: TokenStream) -> TokenStream {
    panic!("test")
}

#[proc_macro_attribute]
pub fn my_attribute_panics(_args: TokenStream, _input: TokenStream) -> TokenStream {
    panic!("test")
}
