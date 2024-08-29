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