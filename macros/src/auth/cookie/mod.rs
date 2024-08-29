use darling::{Error, FromMeta, ToTokens};
use proc_macro::TokenStream;
use darling::ast::NestedMeta;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Stmt};


#[derive(FromMeta)]
struct AuthParams {
  key: String,
  #[darling(default)]
  roles: String
}

pub fn validate(args: TokenStream, input: TokenStream) -> TokenStream {
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
  let session_var = get_session_var_token(&item_fn);
  let session = parse_macro_input!(session_var as Ident);

  // Generate new statements for the resultant function
  let statements = generate_statements(attr_args.key, session, attr_args.roles);

  // Preend statements into block 
  item_fn.block.stmts.splice(0..0, statements); 
  item_fn.to_token_stream().into()

}

fn get_session_var_token(item_fn: &ItemFn) -> TokenStream {
  let fn_args: Vec<&FnArg> = item_fn.sig.inputs.iter().collect();
  let mut found = false;
  let mut tmp_string: TokenStream = TokenStream::new();

  for v in fn_args {
    for token in v.into_token_stream().into_iter() {
      match token.to_string().as_str() {
        "Session" => {
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
    panic!("Session is not used in the function signature")
  }

}


// Add statemets that retriebe the token from header and validates if it contains the required role
fn generate_statements(cookie_key: String, session: Ident, roles: String) -> Vec<Stmt> {
  let mut stmts: Vec<Stmt> = vec![];

  stmts.push(parse_quote!{
    let user_roles = match cataclysm_auth::auth::cookie::extract_roles(#cookie_key, #session) {
      Ok(us) => us,
      Err(e) => {
        return Response::forbidden().body(e.to_string());
      }
    };
  });

  stmts.push(parse_quote!{
    if let Err(e) = cataclysm_auth::auth::cookie::validate_access(user_roles, #roles) {
      return Response::forbidden().body(e.to_string());
    };
  });

  stmts
}