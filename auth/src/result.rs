use std::result;

pub enum Error {
  CredentialMissing,
  Unauthorized
}

pub type Result<T> = result::Result<T, Error>;
