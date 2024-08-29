mod auth;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn jwt_protect(args: TokenStream, input: TokenStream) -> TokenStream {
  auth::jwt::validate_jwt(args, input)
}

#[proc_macro_attribute]
pub fn cookie_protect(args: TokenStream, input: TokenStream) -> TokenStream {
  auth::cookie::validate(args, input)
}