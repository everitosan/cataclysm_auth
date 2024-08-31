use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Invalid credential for this resource")]
  CredentialMissing,
  #[error("Malformed credential: `{0}`")]
  BadCredentialReceived(String),
  #[error("Can't create credential: `{0}`")]
  BadCredential(String),
  #[error("Unauthorized for this resource")]
  Unauthorized
}

pub type Result<T> = result::Result<T, Error>;
