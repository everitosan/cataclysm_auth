use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Invalid credential for this resource")]
  CredentialMissing,
  #[error("Unauthorized for this resource")]
  Unauthorized
}

pub type Result<T> = result::Result<T, Error>;
