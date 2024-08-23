use jsonwebtoken::{encode, decode, DecodingKey, Validation, Algorithm, errors::Error};
use serde::de::DeserializeOwned;

pub fn validate_jwt<T>(jwt: String, secret: String) -> Result<T, Error> where T: DeserializeOwned {
  
  let decoded = decode::<T>(
    &jwt,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS512)
  )?;

  Ok(decoded.claims)
}