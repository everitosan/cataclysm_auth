use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use super::super::dtos::application::CreateDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
  pub id: i32,
  pub name: String,
  pub active: bool,
  pub created: NaiveDateTime
}

#[async_trait]
pub trait ApplicationRepo {
  async fn list(&self) -> Result<Vec<Application>>;
  async fn find_by_id(&self, id: i32) -> Result<Option<Application>>;
  async fn save(&self, dto: CreateDto) -> Result<Application>;
  async fn delete(&self, id: i32) -> Result<()>;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn application_structure() {

    let app = Application {
      id: 2,
      name: "SatID".to_owned(),
      active: true,
      created: NaiveDateTime::parse_from_str("2024-07-17 04:18:12.811411", "%Y-%m-%d %H:%M:%S%.f").unwrap()
    };

    assert_eq!(app.name, "SatID".to_owned());
    assert!(app.active);
  } 
}