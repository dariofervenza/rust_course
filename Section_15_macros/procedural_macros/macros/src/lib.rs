
// ATTRIBUTE MACRO EXAMPLE:

// first we add to the toml syn = {version = "2.0", features = ["full"]} and other things

extern crate proc_macro;  // it is used to import the proc_macro statement that we just implemented in the toml --> it is a rust library with the necessary types and traits to define and implement procedural macros

use proc_macro::TokenStream;  // TokenStream is a type that represents a sequence of tokens and it is used as the output of a procedural macro
// remembr that a procedural macro takes some input code, manipulates it and returns a token string representing the modified code

use quote::quote;  // is a macro that can generate a token stream from a syntax tree
// so allows you to construct rust code using rust syntax

use syn::{ parse_macro_input, ItemFn};  // syn is an third party crate that provides a parser for rust syntax
// so it parsers rust code into a syntax tree that can be manipulated by a procedural macro


// so syn --> treee --> quote --> token stream

// lets create our own ATTRIBUTE MACRO called debug_print --> will print debug info aobut ou functions

#[proc_macro_attribute]  // tell rust that we are building our custom attribute macro
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream {  // we will ignore _attr and the actual code will be in the item argument
    // the rust code to modify is in item
    let mut item_fn = parse_macro_input!(item as ItemFn);
    // add a debug print statement
    let ident = &item_fn.sig.ident;  // .sig is the signature and returns a referecence to the function signature (name, arguments, return type)
    // the .dent is the identity, this is the function's name (it is represented with the Ident type)

    item_fn.block.stmts.insert(  // add to the code some lines on its statements (stmts)
        0,
        syn::parse_quote!(
            println!("Entering the function: {}", stringify!(#ident));
        ),
    );
    TokenStream::from(quote! {
        #item_fn
    })
}


