# Cataclysm_Auth

Library to manage authentication and authorization over Cataclysm Branches.

## Installation

```toml
[dependencies]
tokio = "1.39.3"
cataclysm = "0.3.0-beta.2"
cataclysm-auth = { git = "https://github.com/everitosan/cataclysm_auth", version = "0.1.0" }
```

## How to use.
### 🍪 Cookie
To protect a branch with a cookie, you should use the `cookie_protect` macro.

```rust
use cataclysm_auth::cookie_protect;

#[cookie_protect(key="roles", roles="admin")]
async fn only_for_admin(session: Session) -> Response {
  let message = format!("Hello super admin!");
  Response::ok().body(message)
}
```
- **key** refers to the key of the cookie where roles are setted 
- **roles** is a list of allowed roles for this branch separated by a comma: *"admin, visor, super, user"* 

The library exposes way to create a CookieSession that can be parametrized by the use of [env-variables](#cookie-env).

```rust
use cataclysm_auth::auth::cookie::CookieSession;

let branch = ...;

let server = Server::builder(branch)
  .session_creator(CookieSession::from_env())
  .build().unwrap();

```

To see a full working example you can inspect and run [this example](./src/bin/cookie-server.rs).


```bash
$ cargo run --bin cookie-server
````

#### Cookie-env

| Name | Description | Default |
|--|--|--|
| CATACLYSM_AUTH_COOKIE_SECRET | **Required** secret used to create and validate cookies | - |
| CATACLYSM_AUTH_COOKIE_NAME | Name for the cookie | cataclysm-auth |
| CATACLYSM_AUTH_COOKIE_DOMAIN | Cookie domain attribute | localhost |
| CATACLYSM_AUTH_COOKIE_MAX_AGE | Lifetime of cookie in seconds | 60 |
| CATACLYSM_AUTH_COOKIE_SECURE | Cookie secure attribute | true |

### JWT tokens

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
- **roles** is a list of allowed roles for this branch separated by a comma: *"admin, visor, super, user"* 


The library exposes a method to create a JWT that can be parametrized by the use of [env-variables](#jwt-env).

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

To see a full working example you can inspect and run [this example](./src/bin/jwt-server.rs).

```bash
$ cargo run --bin jwt-server
```

#### JWT-env

| Name | Description | Default |
|--|--|--|
| CATACLYSM_AUTH_SECRET | Secret used to create and validate tokens | " " |
| CATACLYSM_AUTH_EXPRATION | Lifetime of token in minutes | 5|

