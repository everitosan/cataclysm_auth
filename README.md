# Cataclysm_Auth

Library to manage authentication and authorization over Cataclysm Branches.

## How to use it.

To protect a branch with a jwt validation, you should use the `jwt_protect` macro.
```rust
use cataclysm_auth::jwt_protect;
use cataclysm::http::{Request, Response};

#[jwt_protect(prefix="Bearer", roles="admin")]
async fn only_for_admin(req: Request) -> Response {
  let message = format!("Hello admin {}!", claim.sub);
  Response::ok().body(message)
}
```
- **prefix** refers to the string placed before the JWT token 
- **roles** list of roles allowed for this branch separaed by a comma: *"admin, visor, super, user"* 


The library exposes a method to create a JWT that can be parametrized by the use of [env-variables](#env-variables).

```rust
use cataclysm_auth::auth::jwt::{create, TokenType};

let jwt_token = create(
    TokenType::Access, 
    "SAHE922A".to_owned(),
    vec!["admin".to_owned(), "visor".to_owned()]
  ).unwrap();
```

Internally defines and uses a BasicClaim struct.
```rust
pub struct BasicClaim {
  pub exp: usize,
  pub sub: String,
  pub roles: Vec<String>
}
``` 

## Env-variables

| Name | Description | Default |
|--|--|--|
| CATACLYSM_AUTH_SECRET | Secret used to create and validate tokens | " " |
| CATACLYSM_AUTH_EXPRATION | Lifetime of token in minutes | 5|

To see a full working example you can inspect and run [this example](./src/main.rs).