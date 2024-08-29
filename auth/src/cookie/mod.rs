use std::{env, time::Duration};

use cataclysm::session::Session;
use crate::result::{Result, Error};

// This function gets content of a specific key from a cookie and split it by comma ','
pub fn extract_roles(key: &str, session: Session) -> Result<Vec<String>> {
  let str_roles = match session.get(key) {
    Some(roles) => { roles },
    None => {
      return Err(Error::CredentialMissing)
    }
  };
  let user_raw_roles = str_roles.split(",").collect::<Vec<&str>>();
  let user_roles: Vec<String> = user_raw_roles.into_iter().map(|role| role.trim().to_string()).collect();
  Ok(user_roles)
}

// This function validates roles extraced from a cookie are contained in allowed roles
pub fn validate_access(user_roles: Vec<String>,  all_allowed_roles: &str) -> Result<()> {
  let allowed_roles = all_allowed_roles.split(",").collect::<Vec<&str>>();
  for role in allowed_roles {
    let r = role.trim().to_string();
    if user_roles.contains(&r) {
      return Ok(())
    }
  }
  Err(Error::Unauthorized)
}


struct CookieConfig {
  name: String,
  secret: String,
  domain: String,
  max_age: Duration,
  secure: bool
}

impl CookieConfig {
  fn default() -> Self {
    let secret = env::var("CATACLYSM_AUTH_COOKIE_SECRET").expect("CATACLYSM_AUTH_COOKIE_SECRET env var is missing");
    let name = env::var("CATACLYSM_AUTH_COOKIE_NAME").unwrap_or("cataclysm-auth".to_owned());
    let mut domain = env::var("CATACLYSM_AUTH_COOKIE_DOMAIN").unwrap_or("localhost".to_string());
    let max_age_str = env::var("CATACLYSM_AUTH_COOKIE_MAX_AGE").unwrap_or("60".to_owned());
    let secure_str = env::var("CATACLYSM_AUTH_COOKIE_SECURE").unwrap_or("true".to_owned());

    let max_age_int = max_age_str.parse::<u64>().unwrap_or(60);
    let max_age = Duration::from_secs(max_age_int);
    let secure = secure_str.to_lowercase() == "true";

    if domain == "" {
      domain = "localhost".to_owned()
    }

    CookieConfig {
      secret,
      name,
      domain,
      secure,
      max_age
    }
  }
}

pub struct CookieSession {}

impl CookieSession {
  pub fn from_env() -> cataclysm::session::CookieSession {
    let config = CookieConfig::default();
    cataclysm::session::CookieSession::new()
      .secret(config.secret)
      .secure(config.secure)
      .max_age(config.max_age as Duration)
      .domain(config.domain)
      .name(config.name)
  }
}