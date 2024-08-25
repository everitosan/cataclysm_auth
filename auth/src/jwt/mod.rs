use std::env;
use chrono::Utc;
use cataclysm::http::Request;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

struct JwtConfig {
  secret: String,
  expiration: i64
}

impl JwtConfig {
  fn default() -> Self {
    let secret = env::var("CATACLYSM_AUTH_SECRET").unwrap_or_default();
    let expiration_str = env::var("CATACLYSM_AUTH_EXPRATION").unwrap_or("5".to_owned());
    let expiration = expiration_str.parse::<i64>().unwrap_or(5);

    JwtConfig {
      secret,
      expiration
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BasicClaim {
  pub exp: usize,
  pub sub: String,
  pub roles: Vec<String>
}

pub enum TokenType {
  Refresh,
  Access
}

// This function validates a JWT
pub fn validate(jwt: String) -> Option<BasicClaim> {
  let config = JwtConfig::default();
  match decode::<BasicClaim>(
    &jwt,
    &DecodingKey::from_secret(config.secret.as_bytes()),
    &Validation::new(Algorithm::HS512)
  ) {
    Ok(decoded) => {
      Some(decoded.claims)
    },
    Err(_) => {
      None
    }
  }
}

// This function creates a Basic JWT based in env vars 
pub fn create(token_type: TokenType, sub: String, roles: Vec<String>) -> Option<String> {
  let config = JwtConfig::default();
  let duration = match token_type {
    TokenType::Refresh => {
      chrono::Duration::minutes(config.expiration)
    },
    TokenType::Access => {
      chrono::Duration::minutes(config.expiration + 1)
    }
  };

  let expiration = Utc::now()
    .checked_add_signed(duration)
    .expect("invalid timestamp")
    .timestamp();

  let claims = BasicClaim {
    sub,
    roles,
    exp: expiration as usize
  };

  let header = Header::new(Algorithm::HS512);
  if let Ok(f) = encode(&header, &claims, &EncodingKey::from_secret(config.secret.as_bytes())) {
    return Some(f)
  }

  None
}

// This function extracts a JWT from Authorization header
pub fn extact_from_request(req: Request, prefix: &str) -> Option<String> {
  let all_authorizations = req.headers.get("Authorization")?;

  let required_header = all_authorizations
    .into_iter()
    .filter(|h| h.contains(prefix))
    .collect::<Vec<&std::string::String>>();

  if required_header.len() == 0 {
    return None
  }

  let splited: Vec<&str> = required_header[0].split(prefix).collect();

  let tkn = splited.get(1)?;
  let response = format!("{}", tkn.trim());
  Some(response)
}

pub fn validate_access(token: String, all_allowed_roles: &str) -> Option<BasicClaim> {
  let claim = validate(token)?;

  if all_allowed_roles != "" {
    let allowed_roles = all_allowed_roles.split(",").collect::<Vec<&str>>();
    for role in allowed_roles {
      let r = role.trim().to_string();
      if claim.roles.contains(&r) {
        return Some(claim)
      }
    }
    return None
  }

  Some(claim)
}