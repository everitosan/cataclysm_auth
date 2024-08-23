mod auth;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn auth(args: TokenStream, input: TokenStream) -> TokenStream {
  // auth::guard::validate_jwt(args, input)
  auth::jwt::macros::validate_jwt(args, input)
}