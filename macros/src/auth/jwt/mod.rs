/*
This macro is designed to work with JWT tokens
 */
use darling::{Error, FromMeta, ToTokens};
use proc_macro::TokenStream;
use darling::ast::NestedMeta;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Stmt};

#[derive(FromMeta)]
struct AuthParams {
  prefix: String,
  #[darling(default)]
  roles: String
}

pub fn validate_jwt(args: TokenStream, input: TokenStream) -> TokenStream {
  // Initial parse of TokenStreams
  let args_list = match NestedMeta::parse_meta_list(args.into()) {
    Ok(v) => v,
    Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
  };
  let mut item_fn = parse_macro_input!(input as ItemFn);
  
  let attr_args = match AuthParams::from_list(&args_list) {
    Ok(v) => v,
    Err(e) => { return TokenStream::from(e.write_errors()) }
  };

  // Retrieve the name of catalcysm variable related to Request
  let request_var = get_request_var_token(&item_fn);
  let req = parse_macro_input!(request_var as Ident);

  // Generate new statements for the resultant function
  let statements = generate_statements(req, attr_args.prefix, attr_args.roles);

  // Preend statements into block 
  item_fn.block.stmts.splice(0..0, statements); 
  item_fn.to_token_stream().into()
}

fn get_request_var_token(item_fn: &ItemFn) -> TokenStream {
  let fn_args: Vec<&FnArg> = item_fn.sig.inputs.iter().collect();
  let mut found = false;
  let mut tmp_string: TokenStream = TokenStream::new();

  for v in fn_args {
    for token in v.into_token_stream().into_iter() {
      match token.to_string().as_str() {
        "Request" => {
          found = true;
          break;
        },
        ":" => {},
        _ => {
          tmp_string = token.to_token_stream().into();
        }
      }
    }
  }

  if found {
    return tmp_string
  } else {
    panic!("Request is not used in the function signature")
  }

}

// Add statemets that retriebe the token from header and validates if it contains the required role
fn generate_statements(req: Ident, prefix: String, roles: String) -> Vec<Stmt> {
  let mut stmts: Vec<Stmt> = vec![];

  stmts.push(parse_quote!{
    let token = match cataclysm_auth::auth::jwt::extact_from_request(#req, #prefix) {
      Some(t) => t,
      None => {
        return Response::forbidden();
      }
    };
  });

  stmts.push(parse_quote!{
    let claim = match cataclysm_auth::auth::jwt::validate_access(token, #roles) {
      Ok(c) => c,
      Err(_) => {
        return Response::forbidden();
      }
    };
  });

  stmts
}